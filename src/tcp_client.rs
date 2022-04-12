use std::io;
use std::io::Write;
use std::net::TcpStream;

fn main() ->io::Result<()> {
    //创建可变变量stream连接server端
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    for _ in 0..10 {
        //定义一个String类型的输入
        let mut input = String::new();
        //从标准输入读入一行，读入input里面，如果有问题的话，提示“读取失败”
        io::stdin().read_line(&mut input).expect("Failed to read!");
        //把input读取的内容，转换成bytes后，写到stream流里面去，如果写入失败，提示“写入失败”
        stream.write(input.as_bytes()).expect("Failed to write!");
        //美化输出
        println!("----------------------------------------------");

    }
    Ok(())
}
