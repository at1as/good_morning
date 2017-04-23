# Good Morning

Send a daily text with configurable info


### Usage

```
$ cargo run <twilio_auth_token> <to_number> <prepended_message_text>
```

Example:

```
$ cargo run b3000465822a90bef67011e8fea44fa +15556667788 "Rise and Shine!"
```

Response (Received as text message):

```
Rise and Shine! Currently 16 C and Breezy. Today Partly Cloudy with a high of 16 and low of 0. Today's sunset is at 7:51 pm
```

### Notes

* Requires a Twilio account
* Work in progress

