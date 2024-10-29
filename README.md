# About 

I got tired of typing slow, I got tired of singing the lyrics wrong, I got tired of Browser typing websites, my quick fox did not want to jump the brown dog anymore.
Speed up your typing by practising on your favourite songs. 

- How to set up?

You would need to generate an api key for this api `https://rapidapi.com/Paxsenix0/api/musixmatch-lyrics-songs`, it is called `x-rapid-api-key`. The TUI is using this api to get the lyrics of the song.

Add these two variables in the given format 

## For linux 

`export x_rapid_api_key="YOUR RAPID API KEY HERE"`
`export x_rapid_api_host="YOUR RAPID API HOST KEY HERE"`


## For windows

You can add these to the environment variables make sure you use the correct naming convention because the code would be using these two variables. If they are not present, the application will not work.


# Build

- Clone the branch
`git clone git@github.com:supreetsingh10/lyricist.git`

- Go to the target folder
`cargo build --release`

- Run the application
`cargo run --bin lyricist`

- You can install it using cargo as well, if you want to.


# How to use




