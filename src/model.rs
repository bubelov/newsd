use serde::{Deserialize, Serialize};

/*
  atomFeed =
     element atom:feed {
        atomCommonAttributes,
        (atomAuthor*
         & atomCategory*
         & atomContributor*
         & atomGenerator?
         & atomIcon?
         & atomId
         & atomLink*
         & atomLogo?
         & atomRights?
         & atomSubtitle?
         & atomTitle
         & atomUpdated
         & extensionElement*),
        atomEntry*
     }
*/

#[derive(Serialize, Deserialize)]
pub struct Feed {
    pub id: String,
    pub links: Vec<Link>,
    pub title: String,
}

/*
  atomEntry =
     element atom:entry {
        atomCommonAttributes,
        (atomAuthor*
         & atomCategory*
         & atomContent?
         & atomContributor*
         & atomId
         & atomLink*
         & atomPublished?
         & atomRights?
         & atomSource?
         & atomSummary?
         & atomTitle
         & atomUpdated
         & extensionElement*)
     }
*/

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub content: Option<String>,
    pub id: String,
    pub links: Vec<Link>,
    pub published: Option<String>,
    pub summary: Option<String>,
    pub title: String,
    pub updated: String,
}

/*
  atomLink =
     element atom:link {
        atomCommonAttributes,
        attribute href { atomUri },
        attribute rel { atomNCName | atomUri }?,
        attribute type { atomMediaType }?,
        attribute hreflang { atomLanguageTag }?,
        attribute title { text }?,
        attribute length { text }?,
        undefinedContent
     }
*/

#[derive(Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    pub rel: Option<String>,
    pub media_type: Option<String>,
    pub href_lang: Option<String>,
    pub title: Option<String>,
    pub length: Option<u64>,
}

impl From<&feed_rs::model::Link> for Link {
    fn from(link: &feed_rs::model::Link) -> Self {
        Link {
            href: link.href.clone(),
            rel: link.rel.clone(),
            media_type: link.media_type.clone(),
            href_lang: link.href_lang.clone(),
            title: link.title.clone(),
            length: link.length,
        }
    }
}
