use std::env;
use dotenvy::dotenv;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::{Mailbox, header::ContentType}, transport::smtp::authentication::Credentials};

pub struct EmailConfig {
    smtp_user: String,
    smtp_pass: String,
    smtp_host: String,
    user_name: String,
}

impl EmailConfig {
    pub fn from_env(env_path: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(path) = env_path {
            dotenvy::from_path(path).ok();
        } else {
            dotenv().ok();
        }
        Ok(Self {
            smtp_user: env::var("SMTP_USER")?,
            smtp_pass: env::var("SMTP_PASS")?,
            smtp_host: env::var("SMTP_HOST")?,
            user_name: env::var("USER_NAME")?,
        })
    }

    pub async fn send(
        &self,
        to_name: &str,
        to_email: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(Mailbox::new(Some(self.user_name.clone()), self.smtp_user.parse()?))
            .reply_to(Mailbox::new(Some(self.user_name.clone()), self.smtp_user.parse()?))
            .to(Mailbox::new(Some(to_name.to_string()), to_email.parse()?))
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())?;

        let creds = Credentials::new(self.smtp_user.clone(), self.smtp_pass.clone());
        let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay(self.smtp_host.as_str())?
            .credentials(creds)
            .build();

        mailer.send(email).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取config对象
    let email_config = EmailConfig::from_env(Some(".env"))?;

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("参数输入错误，请重新输入");
        // return Err(Box::new(Error::new(ErrorKind::InvalidInput,
        //      "用法: cargo run -- <收件人姓名> <收件人邮箱>",)) as Box<dyn std::error::Error>);
        return Err("用法: cargo run -- <收件人姓名> <收件人邮箱>".into());
    }

    let receiver_name = args.get(1).map(|rn| rn.as_str()).unwrap_or("bug0101");
    let receiver_email = args.get(2).map(|re| re.as_str()).unwrap_or("@outlook.com");
    let subject = args.get(3).map(|s| s.as_str()).unwrap_or("【rust_lettre_test】我的邮箱脚本");
    let body = args.get(4).map(|b| b.as_str()).unwrap_or("测试成功！");

    email_config.send(receiver_name, receiver_email, subject, body).await?;
    Ok(())
}



// use std::env;
// use dotenvy::dotenv;
// // 关键：必须引入这个 Trait，异步 send 才能被识别
// use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::{Mailbox, header::ContentType}, transport::smtp::authentication::Credentials};

// // 把你之前的结构体拷过来（记得加 pub 或者直接放在同一个文件）
// struct EmailConfig {
//     smtp_user: String,
//     smtp_pass: String,
//     smtp_host: String,
//     user_name: String,
// }

// impl EmailConfig {
//     // ... 保持你之前的 from_env 不变 ...
//     fn from_env(env_path: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
//         if let Some(path) = env_path { dotenvy::from_path(path).ok(); } else { dotenv().ok(); }
//         Ok(Self {
//             smtp_user: env::var("SMTP_USER")?,
//             smtp_pass: env::var("SMTP_PASS")?,
//             smtp_host: env::var("SMTP_HOST")?,
//             user_name: env::var("USER_NAME")?,
//         })
//     }

//     async fn send(&self, to_name: &str, to_email: &str) -> Result<(), Box<dyn std::error::Error>> {
//         let email = Message::builder()
//             .from(Mailbox::new(Some(self.user_name.clone()), self.smtp_user.parse()?))
//             .to(Mailbox::new(Some(to_name.to_string()), to_email.parse()?))
//             .subject("并发测试")
//             .body("看到这封信说明并发成功".to_string())?;

//         let creds = Credentials::new(self.smtp_user.clone(), self.smtp_pass.clone());
        
//         // 显式标注类型，解决 cannot infer type
//         let mailer: AsyncSmtpTransport<Tokio1Executor> = 
//             AsyncSmtpTransport::<Tokio1Executor>::relay(self.smtp_host.as_str())?
//                 .credentials(creds)
//                 .build();

//         mailer.send(email).await?;
//         Ok(())
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let config = EmailConfig::from_env(Some(".env"))?;

//     println!("🚀 正在启动 M4 并发引擎...");

//     // 重点：不要直接在 join! 里写调用，先声明 Future 变量
//     let task1 = config.send("bug0101", "@outlook.com");
//     let task2 = config.send("bug0102", "@outlook.com");
//     let task3 = config.send("bug0101", "@outlook.com");
//     // 使用 join! 并发执行
//     let (res1, res2, res3) = tokio::join!(task1, task2, task3);

//     res1?;
//     res2?;
//     res3?;
//     println!("✨ 并发测试完成！");
//     Ok(())
// }