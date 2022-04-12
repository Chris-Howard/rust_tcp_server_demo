use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main()->io::Result<()> {
    //println!("Hello, world!");
    //定义一个监听,?等价于except,unwrap
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    //不断接收连接
    for stream in listener.incoming(){
        //转换流，如果出现问题则提示失败
        let stream = stream.expect("失败");
        handle_client(stream).unwrap_or_else(|error|eprintln!("{:?}",error));
    }
    Ok(())
}

//该函数用于处理client的流
fn handle_client(mut stream: TcpStream)->std::io::Result<()>{
    //创建一个可变变量buf，用来保存数据
    let mut buf=[0;512];
    //循环表示服务器一直工作
    loop{
        //从stream读取数据到buf
        match stream.read(&mut buf){
            //如果读取到的数据长度为0，表示关闭连接
            Ok(n) if n==0 => {
                println!("连接关闭")
            },
            //如果读取到了数据，将读取到的数据打印到控制台
            Ok(n) =>{
                println!("收到客户端发送过来的数据:{}",String::from_utf8_lossy(&buf[..n]));
                //美化输出
                println!("----------------------------------------------");
                //将内容写到客户端
                stream.write(&buf[..n])?;
            },
            Err(..)=>{
                //错误
            },
        }
    }
}
