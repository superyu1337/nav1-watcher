#![allow(dead_code)]

use serde::{Deserialize, Serialize};

fn null() -> String {
    String::from("null")
}

#[derive(Deserialize, Serialize)]
pub struct Webhook {
    #[serde(default = "null")]
    content: String,
    embeds: Vec<Embed>,
    username: Option<String>,
    avatar_url: Option<String>
}

impl Webhook {
    pub fn builder() -> WebhookBuilder {
        WebhookBuilder { content: None, embeds: None, username: None, avatar_url: None }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Failed to stringify webhook object")
    }
}

#[derive(Deserialize, Serialize)]
pub struct Embed {
    title: Option<String>,
    description: Option<String>,
    color: Option<usize>,
    fields: Option<Vec<Field>>,
    author: Option<Author>,
    footer: Option<Footer>,
}

impl Embed {
    pub fn builder() -> EmbedBuilder {
        EmbedBuilder { title: None, description: None, color: None, fields: None, author: None, footer: None }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Field {
    name: String,
    value: String,
    inline: Option<bool>,
}

impl Field {
    pub fn builder(name: String, value: String) -> FieldBuilder {
        FieldBuilder { name, value, inline: None }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Author {
    name: String,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

impl Author {
    pub fn builder(name: String) -> AuthorBuilder {
        AuthorBuilder { name, url: None, icon_url: None, proxy_icon_url: None }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Footer {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}


impl Footer {
    pub fn builder(text: String) -> FooterBuilder {
        FooterBuilder { text, icon_url: None, proxy_icon_url: None }
    }
}

pub struct WebhookBuilder {
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    username: Option<String>,
    avatar_url: Option<String>,
}

impl WebhookBuilder {
    pub fn content(mut self, content: String) -> WebhookBuilder {
        self.content = Some(content);
        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> WebhookBuilder {
        self.embeds = Some(embeds);
        self
    }

    pub fn username(mut self, username: String) -> WebhookBuilder {
        self.username = Some(username);
        self
    }

    pub fn avatar_url(mut self, avatar_url: String) -> WebhookBuilder {
        self.avatar_url = Some(avatar_url);
        self
    }

    pub fn build(self) -> Webhook {
        Webhook {
            content: self.content.unwrap_or(String::from("null")),
            embeds: self.embeds.expect("No embeds given to WebhookBuilder"),
            username: self.username,
            avatar_url: self.avatar_url,
        }
    }
}

pub struct EmbedBuilder {
    title: Option<String>,
    description: Option<String>,
    color: Option<usize>,
    fields: Option<Vec<Field>>,
    author: Option<Author>,
    footer: Option<Footer>,
}

impl EmbedBuilder {
    pub fn title(mut self, title: String) -> EmbedBuilder {
        self.title = Some(title);
        self
    }

    pub fn description(mut self, description: String) -> EmbedBuilder {
        self.description = Some(description);
        self
    }

    pub fn color(mut self, color: usize) -> EmbedBuilder {
        self.color = Some(color);
        self
    }

    pub fn fields(mut self, fields: Vec<Field>) -> EmbedBuilder {
        self.fields = Some(fields);
        self
    }

    pub fn author(mut self, author: Author) -> EmbedBuilder {
        self.author = Some(author);
        self
    }

    pub fn footer(mut self, footer: Footer) -> EmbedBuilder {
        self.footer = Some(footer);
        self
    }

    pub fn build(self) -> Embed {
        Embed {
            title: self.title,
            description: self.description,
            color: self.color,
            fields: self.fields,
            author: self.author,
            footer: self.footer,
        }
    }
}

pub struct FieldBuilder {
    name: String,
    value: String,
    inline: Option<bool>,
}

impl FieldBuilder {
    pub fn inline(mut self, inline: bool) -> FieldBuilder {
        self.inline = Some(inline);
        self
    }

    pub fn build(self) -> Field {
        Field { name: self.name, value: self.value, inline: self.inline }
    }
}

pub struct AuthorBuilder {
    name: String,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

impl AuthorBuilder {
    pub fn url(mut self, url: String) -> AuthorBuilder {
        self.url = Some(url);
        self
    }

    pub fn icon_url(mut self, icon_url: String) -> AuthorBuilder {
        self.icon_url = Some(icon_url);
        self
    }

    pub fn proxy_icon_url(mut self, proxy_icon_url: String) -> AuthorBuilder {
        self.proxy_icon_url = Some(proxy_icon_url);
        self
    }

    pub fn build(self) -> Author {
        Author { name: self.name, url: self.url, icon_url: self.icon_url, proxy_icon_url: self.proxy_icon_url }
    }
}

pub struct FooterBuilder {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

impl FooterBuilder {
    pub fn icon_url(mut self, icon_url: String) -> FooterBuilder {
        self.icon_url = Some(icon_url);
        self
    }

    pub fn proxy_icon_url(mut self, proxy_icon_url: String) -> FooterBuilder {
        self.proxy_icon_url = Some(proxy_icon_url);
        self
    }

    pub fn build(self) -> Footer {
        Footer { text: self.text, icon_url: self.icon_url, proxy_icon_url: self.proxy_icon_url }
    }
}