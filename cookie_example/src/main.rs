use std::net::SocketAddr;
use tokio::net::TcpListener;
use sessions::{ MemoryStorage, Value };
use vidi::{
    IntoResponse,
    Request,
    RequestExt,
    Result,
    Router,
    get,
    middleware::{ cookie, helper::CookieOptions, session::{ self, Store } },
    serve,
    types::CookieKey,
};

// set 首页：显示和修改计数器
async fn index(req: Request) -> Result<String> {
    // 获取当前访问次数;
    let mut count = req.session().get::<i32>("count")?.unwrap_or(0);
    count += 1;
    req.session().set("count", count)?;
    Ok(format!("欢迎访问！这是您第 {} 次访问本网站。", count))
}

// set 登录页面：模拟用户登录
async fn login(req: Request) -> Result<String> {
    let username = "user123"; // 模拟用户名
    req.session().set("username", username)?;
    Ok(format!("登录成功！用户名: {}", username))
}

// get 获取用户信息
async fn profile(req: Request) -> Result<String> {
    // 检查是否已登录
    if let Some(username) = req.session().get::<String>("username")? {
        Ok(format!("用户信息: {}", username))
    } else {
        Ok("请先登录！".to_string())
    }
}

// set 添加商品到购物车
async fn add_to_cart(req: Request) -> Result<String> {
    let product = "iPhone 15";

    // 获取购物车（如果没有则创建空数组）
    let mut cart = req.session().get::<Vec<String>>("cart")?.unwrap_or_default();

    // 添加商品
    cart.push(product.to_string());
    req.session().set("cart", &cart)?;

    Ok(format!("商品 '{}' 已添加到购物车！", product))
}

//get 查看购物车
async fn view_cart(req: Request) -> Result<String> {
    if let Some(cart) = req.session().get::<Vec<String>>("cart")? {
        if cart.is_empty() {
            Ok("购物车是空的".to_string())
        } else {
            let items = cart.join(", ");
            Ok(format!("购物车中的商品: {}", items))
        }
    } else {
        Ok("购物车是空的".to_string())
    }
}

//remove  清空购物车
async fn clear_cart(req: Request) -> Result<String> {
    if let Some(_) = req.session().remove("cart") {
        return Ok("购物车已清空！".to_string());
    } else {
        return Err(vidi::StatusCode::INTERNAL_SERVER_ERROR.into_error());
    }
}

//clear  退出登录并且将所有cookie清除
async fn logout(req: Request) -> Result<String> {
    let ok: vidi::Error = req.session().clear().into_error();
    if ok.is::<vidi::Error>() {
        return Err(vidi::StatusCode::INTERNAL_SERVER_ERROR.into_error());
    }
    Ok("您已成功退出登录！".to_string())
}

//  get name 获取所有会话数据
async fn debug(req: Request) -> Result<String> {
    let count: u32 = req.session().get("count")?.unwrap_or(0);
    let username: Option<String> = req.session().get("username")?;
    let cart: Vec<String> = req.session().get("cart")?.unwrap_or_default();

    let mut response = String::new();
    response.push_str("=== 当前会话数据 ===\n");
    response.push_str(&format!("访问次数: {}\n", count));
    response.push_str(&format!("用户名: {}\n", username.unwrap_or("未登录".to_string())));
    response.push_str(&format!("购物车: {}\n", cart.join(", ")));

    Ok(response)
}
//data 获取所有会话数据
async fn debug_all(req: Request) -> Result<String> {
    let data_map = req.session().data()?;
    // 转换为 Vec<String>
    let cart_list_val: Vec<String> = data_map
        .get("cart")
        .and_then(Value::as_array)
        .map(|array| {
            array
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    println!("cart_list_val ==={:?}", cart_list_val);
    let mut response = String::new();
    response.push_str("=== 当前会话数据 ===\n");
    response.push_str(&format!("访问次数: {}\n", data_map.get("count").unwrap_or_default()));
    response.push_str(&format!("用户名: {}\n", data_map.get("cart").unwrap_or_default()));
    response.push_str(&format!("购物车: {}\n", data_map.get("username").unwrap_or_default()));

    Ok(response)
}
//data status 获取 status

async fn debug_status(req: Request) -> Result<String> {
    //  UNCHANGED: u8 = 0 如果会话未更改或已初始化，则设置。
    // PURGED: u8 = 1 如果会话已被销毁，则设置。
    // RENEWED: u8 = 2 如果会话已被更新，则设置。
    // CHANGED: u8 = 3 如果会话已被更改，则设置。
    let data = req.session().status();
    Ok(String::from(format!("{:?}", data)))
}

//data status 更新 status
async fn debug_update_status(req: Request) -> Result<String> {
    //  UNCHANGED: u8 = 0 如果会话未更改或已初始化，则设置。
    // PURGED: u8 = 1 如果会话已被销毁，则设置。
    // RENEWED: u8 = 2 如果会话已被更新，则设置。
    // CHANGED: u8 = 3 如果会话已被更改，则设置。
    let ok: vidi::Error = req.session().renew().into_error();
    if ok.is::<vidi::Error>() {
        return Err(vidi::StatusCode::INTERNAL_SERVER_ERROR.into_error());
    }
    Ok(String::from("状态更新成功"))
}
//remove_as
async fn debug_remove_as(req: Request) -> Result<String> {
    req.session().set::<i32>("test", 10)?;
    let res = req.session().get::<i32>("test")?.unwrap_or(-1);
    if res == -1 {
        println!(" debug_remove_as -1");
        return Err(vidi::StatusCode::INTERNAL_SERVER_ERROR.into_error());
    }
    let data = req.session().remove_as::<i32>("test").unwrap_or(-2);
    if data == -2 {
        println!("debug_remove_as -2");
        return Err(vidi::StatusCode::INTERNAL_SERVER_ERROR.into_error());
    }
    Ok(String::from(format!("删除成功：{}", data)))
}
#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("服务器运行在: http://{}", addr);

    let app = Router::new()
        // 定义路由
        .route("/", get(index))
        .route("/login", get(login))
        .route("/profile", get(profile))
        .route("/add", get(add_to_cart))
        .route("/cart", get(view_cart))
        .route("/clear", get(clear_cart))
        .route("/logout", get(logout))
        .route("/debug", get(debug))
        .route("/debug_status", get(debug_status))
        .route("/debug_update_status", get(debug_update_status))
        .route("/debug_all", get(debug_all))
        .route("/remove_as", get(debug_remove_as))
        // 配置会话中间件
        .with(
            session::Config::new(
                Store::new(
                    MemoryStorage::new(),
                    nano_id::base64::<32>, //OR 使用xid|xid 应该是20位字符串|这个是闭包
                    |sid: &str| sid.len() == 32
                ),
                CookieOptions::default()
            )
        )
        // 配置 Cookie 中间件
        .with(cookie::Config::with_key(CookieKey::generate()));

    // 启动服务器
    if let Err(e) = serve(listener, app).await {
        println!("服务器错误: {}", e);
    }

    Ok(())
}
