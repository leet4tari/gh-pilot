use crate::models::common::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Label {
    pub id: u64,
    pub node_id: String,
    pub url: Url,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub default: bool,
}

#[cfg(test)]
mod test {
    use crate::models::labels::Label;

    const LABELS: &str = r#"
[
    {
        "id": 1189345686,
        "node_id": "MDU6TGFiZWwxMTg5MzQ1Njg2",
        "url": "https://api.github.com/repos/tari-project/tari/labels/in%20progress",
        "name": "in progress",
        "color": "ededed",
        "default": false,
        "description": null
    },
    {
        "id": 1390045826,
        "node_id": "MDU6TGFiZWwxMzkwMDQ1ODI2",
        "url": "https://api.github.com/repos/tari-project/tari/labels/Integration%20test",
        "name": "Integration test",
        "color": "f435f9",
        "default": false,
        "description": ""
    },
    {
        "id": 957513340,
        "node_id": "MDU6TGFiZWw5NTc1MTMzNDA=",
        "url": "https://api.github.com/repos/tari-project/tari/labels/invalid",
        "name": "invalid",
        "color": "e4e669",
        "default": true,
        "description": "This doesn't seem right"
    },
    {
        "id": 4274976216,
        "node_id": "LA_kwDOCCIzW87-zvXY",
        "url": "https://api.github.com/repos/tari-project/tari/labels/javascript",
        "name": "javascript",
        "color": "168700",
        "default": false,
        "description": "Pull requests that update Javascript code"
    },
    {
        "id": 2024296734,
        "node_id": "MDU6TGFiZWwyMDI0Mjk2NzM0",
        "url": "https://api.github.com/repos/tari-project/tari/labels/Known-Issue",
        "name": "Known-Issue",
        "color": "bfdadc",
        "default": false,
        "description": "Issue that has already been discovered"
    }
]
    "#;
    #[test]
    fn deserialize_labels() {
        let labels: Vec<Label> = serde_json::from_str(LABELS).unwrap();
        assert_eq!(labels.len(), 5);
        assert_eq!(labels[0].id, 1189345686);
        assert_eq!(labels[4].color, "bfdadc");
    }
}
