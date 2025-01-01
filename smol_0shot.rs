/*
PROMPT:
with this documentation, write tokio-tungstenite code with serde_json for deserialization to connect to his websocket and subscribe & deserialize the trade channel eneral
Current Version
Bitfinex Websocket API version is 2.0

URL
Channels that require authentication should use the domain:
wss://api.bitfinex.com/

Public channels should use the domain:
wss://api-pub.bitfinex.com/
ublic Channels
URL
Public channels should use the domain:
wss://api-pub.bitfinex.com/

The domain:
wss://api.bitfinex.com/
Should only be used for channels that require authentication.

Rate Limit
The rate limit for the wss://api-pub.bitfinex.com/ domain is set at 20 connections per minute.

Connecting to a channel
Use a subscription message to subscribe to a channel:

JSON

{
   "event": "subscribe",
   "channel": CHANNEL_NAME
}
Each channel documentation page also includes example subscription messages. You can subscribe to 25 channels at the same time on the same connection.

Ticker
Trades
Books
Raw Books
Candles
Text

// For public channels:
wss://api-pub.bitfinex.com/ws/2

// For authenticated channels:
wss://api.bitfinex.com/ws/2
Message encoding
Each message sent and received via the Bitfinex's websocket channel is encoded in JSON format

All times are UTC timestamps expressed as milliseconds (eg 1477409622229)

❗️
Array Length

Message (JSON array) lengths should never be hardcoded. New fields may be appended at the end of a message without changing version.

Supported Pairs
All pairs available in the platform are supported. A symbol can be a trading pair or a margin currency. All symbols are in uppercase (i.e. btcusd is invalid, BTCUSD is valid).

Trading pairs are prepended by a “t” before the pair (e.g. tBTCUSD, tETHUSD, ...).
Margin currencies are prepended by an “f” before the currency (e.g. fUSD, fBTC, …).

Lists of available currencies and pairs can be retrieved through the Configs endpoint.

For a list of all currencies on the platform, look to:
https://api-pub.bitfinex.com/v2/conf/pub:list:currency
For a list of all trading pairs on the platform, look to:
https://api-pub.bitfinex.com/v2/conf/pub:list:pair:exchange
For a list of all margin trading pairs on the platform, look to:
https://api-pub.bitfinex.com/v2/conf/pub:list:pair:margin

How to Connect
Open up a websocket connection to the websocket URI.

JavaScript
Go

const WebSocket = require('ws')

const wss = new WebSocket('wss://api-pub.bitfinex.com/ws/2')
wss.onmessage = (msg) => console.log(msg.data)
wss.onopen = () => {
  // API keys setup here (See "Authenticated Channels")
}
A connection to one or more Public Channels can be opened by running the example code found on their respective documentation pages.

Instructions on how to establish a connection to an authenticated channel can be found on the Authenticated Channels page.

Instructions on Websocket Inputs can be found on the Websocket Inputs page.

Subscribe to Channels
You can subscribe to channels using a subscribe event. Below is a request response template:

JSON

// request
{
   "event": "subscribe",
   "channel": CHANNEL_NAME
}

// response
{
   "event": "subscribed",
   "channel": CHANNEL_NAME,
   "chanId": CHANNEL_ID
}

// response-failure
{
   "event": "error",
   "msg": ERROR_MSG,
   "code": ERROR_CODE
}
📘
Channel Name and Channel ID

When subscribing to channels, each will be given a CHANNEL_ID and CHANNEL_NAME.

CHANNEL_NAME: (string) channel name (book, trades, ticker)
CHANNEL_ID: (int) channel identifier. CHANNEL_ID is a numeric channel identifier that the developer can use to distinguish between updates for each subscribed channel.
📘
Subscription Limit

All websocket connections have a limit of 30 subscriptions to public market data feed channels (tickers, book, candles, trades, …). We kindly ask all users to adapt their application setup accordingly to split subscriptions to channels using multiple WebSocket connections.

Please note that when using an authenticated connection a channel is reserved for account info messages.

❗️
Subscription Error Codes

10300 : Subscription failed (generic)
10301 : Already subscribed
10302 : Unknown channel
Unsubscribing from Channels
To stop receiving data from a channel you have to send a "unsubscribe" message. Please find a model for this below:

JSON

// request
{
   "event": "unsubscribe",
   "chanId": CHANNEL_ID
}

// response
{
   "event": "unsubscribed",
   "status": "OK",
   "chanId": CHANNEL_ID
}

// response-failure
{
   "event": "error",
   "msg": ERROR_MSG,
   "code": ERROR_CODE
}
❗️
Error Codes

10400 : Subscription failed (generic)
10401 : Not subscribed

Other Error Codes
In case of error, you receive a message containing the proper error code (code JSON field).

A full list of error codes can be found in the Abbreviation Glossary .

❗️
Generic Error Codes

10000 : Unknown event
10001 : Unknown pair
10305 : Reached limit of open channels
Info Messages
Info messages are sent from the websocket server to notify you about the state of your connection.

Right after connecting you will receive an info message that contains the actual version of the websocket stream, along with a platform status flag (1 for operative, 0 for maintenance).

JSON

{
   "event": "info",
   "version":  VERSION,
   "platform": {
      "status": 1
   }
}
📘
NOTE

If you are developing/using a trading bot, please make sure to handle version number changes.

The websocket server sends other info messages to inform regarding relevant events.

JSON

{
   "event":"info",
   "code": CODE,
   "msg": MSG
}
📘
Info Codes

20051 : Stop/Restart Websocket Server (please reconnect)
20060 : Entering in Maintenance mode. Please pause any activity and resume after receiving the info message 20061 (it should take 120 seconds at most).
20061 : Maintenance ended. You can resume normal activity. It is advised to unsubscribe/subscribe again all channels.
🚧
Only rely on 'CODE' for 'info' events

We wish to emphasise to only rely on the event message codes rather than the text descriptions of the event messages. The descriptions may change over time and are not part of the protocol.

Ping/Pong
You can send a ping message to test your connection to the websocket server.

JSON

// request
{
   "event":"ping",
   "cid": 1234
}

// response
{
   "event":"pong",
   "ts": 1511545528111,
   "cid": 1234
}
Snapshot
Upon subscribing to a channel, sometimes an initial snapshot is sent.

Models and examples of snapshots and updates can be found on the documentation page for each channel/event.

Update
After receiving the snapshot, you will receive updates upon any change. Updates will have the same CHANNEL_ID as the initial snapshot and the event confirming the subscription to the channel.

Models and examples of snapshots and updates can be found on the documentation page for each channel/event.

Heartbeating
Every 15 seconds, the Websocket server will send you a heartbeat message in this format.

JSON

[ CHANNEL_ID, "hb" ]
Configuration
Conf events can be used to change settings.

JSON

{ 
  event: "conf", 
  flags: FLAGS
}
📘
Flags

In order to change the configuration, there is a new event able to be requested conf, and this will have a parameter flags which is the bitwise XOR of the different options listed below

If you wish to enable more than one flag, sum up their values.

Available Options for Conf events
Name	Value	Description
TIMESTAMP	32768	Adds a Timestamp in milliseconds to each received event.
SEQ_ALL	65536	Adds sequence numbers to each event. This allows you to see if you are experiencing package loss or if you are receiving messages in a different order than they were sent from our server BETA FEATURE
OB_CHECKSUM	131072	Enable checksum for every book iteration. Checks the top 25 entries for each side of book. Checksum is a signed int.

For more info, see the WebSocket Checksum page
BULK_UPDATES	536870912	Enables receiving multiple book updates in a single message as an array of arrays. Applicable to both Books and Raw Books. Trades
This channel sends a trade message whenever a trade occurs at Bitfinex. It includes all the pertinent details of the trade, such as price, size and the time of execution. The channel can send funding trade data as well.

JavaScript
Shell
Python

const ws = require('ws')
const w = new ws('wss://api-pub.bitfinex.com/ws/2')

w.on('message', (msg) => console.log(msg))

let msg = JSON.stringify({ 
  event: 'subscribe', 
  channel: 'trades', 
  symbol: 'tBTCUSD' 
})

w.on('open', () => w.send(msg))
Request / Response
Snapshot
Update

// request
{ 
  event: "subscribe", 
  channel: "trades", 
  symbol: SYMBOL 
}

// response Trading
{
  event: "subscribed",
  channel: "trades",
  chanId: CHANNEL_ID,
  symbol: "tBTCUSD"
  pair: "BTCUSD"
}

{"event":"subscribed","channel":"trades","chanId":19111,"symbol":"tBTCUSD","pair":"BTCUSD"}

// response Funding
{
  event: "subscribed",
  channel: "trades",
  chanId: CHANNEL_ID,
  symbol: "fUSD",
  currency: "USD"
}

{"event":"subscribed","channel":"trades","chanId":339521,"symbol":"fUSD","currency":"USD"}
Request fields
Fields	Type	Description
SYMBOL	String	Trading pair or funding currency
Trade snapshot data
Index	Field	Type	Description
[0]	CHANNEL_ID	Int	Identification number assigned to the channel for the duration of this connection.
[1]	SNAPSHOT	Array	Array with an array of recent trades (Indices [0...n] will be trades)
[1][0...n]	TRADE	Array	Trade array or funding trade array
Trade update data
Index	Field	Type	Description
[0]	CHANNEL_ID	Int	Identification number assigned to the channel for the duration of this connection.
[1]	MSG_TYPE	String	"te" (trade executed), "tu" (trade updated), "fte" (funding trade executed), "ftu" (funding trade updated
[2]	TRADE	Trade	Trade array or funding trade array
Trade arrays
Index	Field	Type	Description
[0]	ID	Int	Trade ID
[1]	MTS	Int	Millisecond time stamp
[2]	AMOUNT	Float	Amount bought (positive) or sold (negative).
[3]	PRICE	Float	Price at which the trade was executed
Funding trade arrays
Index	Field	Type	Description
[0]	ID	Int	Trade ID
[1]	MTS	Int	Millisecond time stamp
[2]	AMOUNT	Float	Amount of funding provided (positive) or taken (negative).
[3]	RATE	Float	Funding rate of the trade
[4]	PERIOD	Int	Funding offer period in days
📘
Amount

The order that causes the trade (the taker) determines if it is a buy or a sell (for trades) or if funding is. Tokio tungstenite example: use futures_util::{SinkExt, StreamExt};
use log::*;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error, Result},
};

const AGENT: &str = "Tungstenite";

async fn get_case_count() -> Result<u32> {
    let (mut socket, _) = connect_async("ws://localhost:9001/getCaseCount").await?;
    let msg = socket.next().await.expect("Can't fetch case count")?;
    socket.close(None).await?;
    Ok(msg.to_text()?.parse::<u32>().expect("Can't parse case count"))
}

async fn update_reports() -> Result<()> {
    let (mut socket, _) =
        connect_async(&format!("ws://localhost:9001/updateReports?agent={}", AGENT)).await?;
    socket.close(None).await?;
    Ok(())
}

async fn run_test(case: u32) -> Result<()> {
    info!("Running test case {}", case);
    let case_url = &format!("ws://localhost:9001/runCase?case={}&agent={}", case, AGENT);
    let (mut ws_stream, _) = connect_async(case_url).await?;
    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let total = get_case_count().await.expect("Error getting case count");

    for case in 1..=total {
        if let Err(e) = run_test(case).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => error!("Testcase failed: {}", err),
            }
        }
    }

    update_reports().await.expect("Error updating reports");
}

*/
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tungstenite::{Message, Error};
use serde_json::{json, Value};
use log::{info, error};

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let endpoint = "wss://api-pub.bitfinex.com/ws/2";
    
    match connect_async(endpoint).await {
        Ok((socket, _)) => {
            info!("Connected to websocket");
            
            // Subscribe to trades channel
            let subscription = json!({
                "event": "subscribe",
                "channel": "trades",
                "symbol": "tBTCUSD"
            });
            
            socket.send(Message::Text(subscription.to_string())).await?;
            
            // Consume messages and process them
            let (tx, mut rx) = mpsc::channel(100);
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    match msg {
                        Message::Text(text) => {
                            let json: Value = serde_json::from_str(&text).unwrap();
                            process_message(json);
                        },
                        _ => {}
                    }
                }
            });
        },
        Err(e) => {
            error!("Failed to connect to websocket: {}", e);
        }
    }
}

fn process_message(json: Value) {
    info!("Received message: {:?}", json);
    
    // Example processing
    let event = json["event"].as_str().unwrap();
    if event == "subscribed" {
        let chan_id = json["chanId"].as_i64().unwrap();
        info!("Subscribed to channel with ID {}", chan_id);
    } else if event == "message" {
        // Handle other message types
        let trade_data = json["data"].as_array().unwrap();
        for trade in trade_data {
            let id = trade["id"].as_i64().unwrap();
            let mts = trade["mts"].as_u64().unwrap();
            let amount = trade["amount"].as_f64().unwrap();
            let price = trade["price"].as_f64().unwrap();
            info!("Trade: ID {}, MTS: {}, Amount: {}, Price: {}", id, mts, amount, price);
        }
    } else {
        error!("Received unknown message event: {}", event);
    }
}
