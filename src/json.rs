use serde_json::{Value, json};

#[derive(Debug, Clone, Copy)]
pub enum Status<'j> {
    Offline,
    Disconnected {
        lockdown: bool,
    },
    Connected {
        lockdown: bool,
        tunnel_type: &'j str,
        tunnel_interface: &'j str,
        ipv4: Option<&'j str>,
        ipv6: Option<&'j str>,
        country: &'j str,
        city: &'j str,
        hostname: &'j str,
    },
}

impl<'j> Status<'j> {
    pub fn from_status_json(json: &'j Value) -> Option<Self> {
        match json.get("state").and_then(Value::as_str)? {
            "error" => {
                if json
                    .get("details")
                    .and_then(|v| v.get("cause"))
                    .and_then(|v| v.get("reason"))
                    .and_then(Value::as_str)
                    == Some("is_offline")
                {
                    Some(Self::Offline)
                } else {
                    None
                }
            }
            "connected" => {
                let details = json.get("details")?;

                let endpoint = details.get("endpoint")?;
                let tunnel_type = endpoint.get("tunnel_type").and_then(Value::as_str)?;
                let tunnel_interface = endpoint.get("tunnel_interface").and_then(Value::as_str)?;

                let location = details.get("location")?;
                let ipv4 = location.get("ipv4").and_then(Value::as_str);
                let ipv6 = location.get("ipv6").and_then(Value::as_str);
                let country = location.get("country").and_then(Value::as_str)?;
                let city = location.get("city").and_then(Value::as_str)?;
                let hostname = location.get("hostname").and_then(Value::as_str)?;

                let lockdown = details
                    .get("feature_indicators")
                    .and_then(Value::as_array)
                    .map(|vec| vec.iter().any(|v| v.as_str() == Some("LockdownMode")))?;

                Some(Self::Connected {
                    lockdown,
                    tunnel_type,
                    tunnel_interface,
                    ipv4,
                    ipv6,
                    country,
                    city,
                    hostname,
                })
            }
            "disconnected" => {
                let lockdown = json
                    .get("details")
                    .and_then(|j| j.get("locked_down"))
                    .and_then(Value::as_bool)?;

                Some(Self::Disconnected { lockdown })
            }
            _ => None,
        }
    }

    fn alt(&self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Connected { lockdown: true, .. } => "connected-lockdown",
            Self::Connected {
                lockdown: false, ..
            } => "connected",
            Self::Disconnected { lockdown: true, .. } => "disconnected-lockdown",
            Self::Disconnected {
                lockdown: false, ..
            } => "disconnected",
        }
    }

    fn class(&self) -> Vec<&'static str> {
        let mut classes = Vec::new();

        if matches!(self, Self::Offline { .. }) {
            classes.push("offline");
        }
        if matches!(self, Self::Connected { .. }) {
            classes.push("connected");
        }
        if matches!(
            self,
            Self::Connected {
                tunnel_interface: "wireguard",
                ..
            }
        ) {
            classes.push("wireguard");
        }
        if matches!(
            self,
            Self::Connected { lockdown: true, .. } | Self::Disconnected { lockdown: true, .. }
        ) {
            classes.push("lockdown");
        }

        classes
    }

    fn text(&self) -> String {
        match self {
            Self::Offline | Self::Disconnected { .. } => String::new(),
            Self::Connected { hostname, .. } => {
                let mut parts = hostname.split('-');
                let country_code = parts.next().unwrap_or("");
                let city_code = parts.next().unwrap_or("");
                format!(
                    " {},{}",
                    city_code.to_uppercase(),
                    country_code.to_uppercase()
                )
            }
        }
    }

    fn tooltip(&self) -> String {
        match self {
            Self::Offline => "<b>OFFLINE</b>".into(),
            Self::Disconnected { lockdown, .. } => format!("<b>LOCKDOWN: {lockdown}</b>"),
            Self::Connected {
                lockdown,
                tunnel_type,
                tunnel_interface,
                ipv4,
                ipv6,
                country,
                city,
                hostname,
            } => format!(
                "<b>LOCKDOWN: {lockdown}</b>\n\n\
                 Tunnel protocol: {tunnel_type}\n\
                 Tunnel interface: {tunnel_interface}\n\n\
                 IPv4: {}\n\
                 IPv6: {}\n\
                 Location: {city}, {country}\n\
                 Hostname: {hostname}",
                ipv4.unwrap_or("N/A"),
                ipv6.unwrap_or("N/A")
            ),
        }
    }

    pub fn into_response_json(self) -> String {
        let text = self.text();
        let alt = self.alt();
        let class = self.class();
        let tooltip = self.tooltip();

        json!({
            "text": text,
            "alt": alt,
            "tooltip": tooltip,
            "class": class,
        })
        .to_string()
    }
}
