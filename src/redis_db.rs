use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<String> {
    // connect to redis
    let client = redis::Client::open("redis://:000415@192.168.0.49/1")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set_ex("my_key", 42, 10)?;

    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}

fn set_an_integer() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://:000415@192.168.0.49/")?;

    let mut con = client.get_connection()?;
    let res = con.set("hello", "rust redis")?;

    Ok(res)
}

pub async fn hitokoto() -> std::io::Result<String> {
    let client = reqwest::Client::new()
        .get("https://v1.hitokoto.cn/")
        .send()
        .await
        .unwrap();

    let body = client.text().await.unwrap();
    println!("{}", body);
    Ok(body)
}

async fn set_hitokoto() -> std::io::Result<()> {
    let client = redis::Client::open("redis://:000415@192.168.0.49/").unwrap();
    let mut con = client.get_connection().unwrap();
    let hitokoto = hitokoto().await.unwrap();
    let res = con.set("hitokoto", hitokoto).unwrap();
    Ok(res)
}

async fn get_hitokoto() -> std::io::Result<String> {
    let client = redis::Client::open("redis://:000415@192.168.0.49/").unwrap();
    let mut con = client.get_connection().unwrap();
    let res = con.get("hitokoto").unwrap();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_redis() {
        let res = set_an_integer();
        assert_eq!(Ok(()), res);
    }

    #[test]
    fn test_get_string() {
        let res = fetch_an_integer();
        assert_eq!(Ok("42".to_string()), res);
    }
}
