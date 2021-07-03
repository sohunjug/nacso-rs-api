pub mod config;
pub mod instance;

use async_trait::async_trait;
use lazy_static::lazy_static;
use reqwest::{Client, Response};
use serde::Serialize;
use std::error::Error;
use std::time::Duration;

use crate::client::NacosClient;

lazy_static! {
    pub(crate) static ref CLIENT: Client = Client::new();
}

pub trait Nacos {
    fn get_token(&self) -> String;
    fn get_nacos(&self) -> &Option<Box<NacosClient>>;
    fn set_nacos(&mut self, nacos: &NacosClient);
}

#[async_trait]
pub trait Get: Nacos {
    const URI: &'static str = "/";

    async fn get(&self) -> Result<Response, Box<dyn Error>>
    where
        Self: Serialize,
    {
        // let token = match self.get_token() {
        //         None => {},
        //         Some(t) => &[("token", t.to_string()); 1],
        // };
        let token = self.get_token();
        let res = if token == "" {
            CLIENT.get(self.get_nacos().as_ref().unwrap().addr(Self::URI)).query(&self)
        } else {
            CLIENT.get(self.get_nacos().as_ref().unwrap().addr(Self::URI)).query(&[("accessToken", token)]).query(&self)
        }.timeout(Duration::from_secs(10));
        println!("{:?}",res);
        let resp = res.send().await?;
        println!("{:?}", resp);
        Ok(resp)
    }
}

#[async_trait]
pub trait Post: Nacos {
    const URI: &'static str = "/";

    async fn post(&self) -> Result<Response, Box<dyn Error>>
    where
        Self: Serialize,
    {
        let token = self.get_token();
        let res = if token == "" {
            CLIENT.post(self.get_nacos().as_ref().unwrap().addr(Self::URI)).query(&self)
        } else {
            CLIENT.post(self.get_nacos().as_ref().unwrap().addr(Self::URI)).query(&[("accessToken", token)]).query(&self)
        }.timeout(Duration::from_secs(10));
        println!("{:?}",res);
        let resp = res.send().await?;
        println!("{:?}",resp);
        Ok(resp)
        // let resp = CLIENT
        //     .post(self.get_nacos().unwrap().addr(Self::URI))
        //     .query(&self)
        //     .timeout(Duration::from_secs(10))
        //     .send()
        //     .await?;
        // Ok(resp)
    }
}

#[async_trait]
pub trait Put: Nacos {
    const URI: &'static str = "/";

    async fn put(&self) -> Result<Response, Box<dyn Error>>
    where
        Self: Serialize,
    {
        let resp = CLIENT
            .put(self.get_nacos().as_ref().unwrap().addr(Self::URI))
            .query(&self)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;
        Ok(resp)
    }
}

#[async_trait]
pub trait Delete: Nacos {
    const URI: &'static str = "/";

    async fn delete(&self) -> Result<Response, Box<dyn Error>>
    where
        Self: Serialize,
    {
        let resp = CLIENT
            .delete(self.get_nacos().as_ref().unwrap().addr(Self::URI))
            .query(&self)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;
        Ok(resp)
    }
}