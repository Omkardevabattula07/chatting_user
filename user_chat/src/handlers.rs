use actix_web:: Result;
use actix_files::NamedFile;




pub async fn index() -> Result<NamedFile>  {


    Ok(NamedFile::open("./static/index.html")?)
}


pub async fn login() -> Result <NamedFile> {


    Ok(NamedFile::open("./static/login.html")?)
}

pub async fn register() -> Result <NamedFile> {
    Ok(NamedFile::open("./static/register.html")?)
}