# Good Morning

Send a daily text (or MMS) with configurable info (weather, stocks, etc)


### Usage

```
$ cargo run <twilio_auth_token> <to_number>
```

Example:

```
# Using Cargo
$ cargo run b3000465822a90bef67011e8fea44fa +15556667788

# Running binary directly without cargo:
$ ./target/debug/good_morning b3000465822a90bef67011e8fea44fa +15556667788
```

Response (Received as SMS):

```
Rise and Shine! Currently 16 C and Breezy. Today Partly Cloudy with a high of 16 and low of 0. Today's sunset is at 7:51 pm.
AMZN Range: 912.11-921.86
```

### Configuration

```
src/conf/
├── cloudinary_conf.json
├── message_conf.json
└── twilio_conf.json
```

`src/cloudinary_conf.json`:
* upload_preset for accessing cloudinary API to upload MMS images
`src/message_conf.json`
* message settings (destination number, city location, etc)
* toggle SMS and/or MMS delivery
`src/twilio_conf.json`
* account settings (Account SID, Twilio From Number)



### Scheduling

Want a message every weekday morning at 8 a.m.? The easiest way is to an cronjob entry like the following (use `crontab -e` to edit):

```
00 08 * * 1-5 ( cd ~/good_morning/ ; RUST_BACKTRACE=1 ~/good_morning/target/debug/good_morning <TWILIO_ACCOUNT_SID> <DEST_NUMBER> >> /tmp/good_morning_runlog.txt 2>&1  )

Example:
  00 08 * * 1-5 ( cd ~/good_morning/ ; RUST_BACKTRACE=1 ~/good_morning/target/debug/good_morning b3000465822a90bef67011e8fea44fae +15556667890 >> /tmp/good_morning_runlog.txt 2>&1  )
```

Notes:
* Use `crontab -l` to verify entry has been created
* See `/tmp/good_morning_runlog.txt` for trace and successful responses. /tmp/ is regularly cleared of its contents, so choose another directory for long term logs
* Replace `~/good_morning` to your source directory for this project


### Notes

* Requires a [Twilio](https://www.twilio.com) account for SMS and MMS messages
* SMS Character limit is 1600
* Requires a [Cloudinary](https://www.cloudinary.com) account for hosting MMS message images

### TODO

* Allow multiple providers to host images
* Improve MMS image

