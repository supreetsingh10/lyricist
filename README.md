# About 

I got tired of typing slow, I got tired of singing the lyrics wrong, I got tired of browser typing websites, my quick fox did not want to jump the brown dog anymore.
Speed up your typing by practising on your favourite songs. 

## How to set up?

- Make an account on `https://rapidapi.com/`

- You would need to generate an api key for the api `https://rapidapi.com/Paxsenix0/api/musixmatch-lyrics-songs`, and subscribe the given api. Add the `x-rapid-api-key` & `x-rapid-api-host` to your environment variables. 

- The format is given the next section

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


# How to use

The big rectangle in the center will be your friend, it will be used to display messages in case some exceptions arise

- Response body is not deserializing.
- The song searched is not available.

Hit `CTRL-s` to search for the song. The search request is supposed to be in a particular format `t: <Artist Name>, a: <Album Name>`
- Example `t: Black Sabbath, a: Black Sabbath`

The lyrics of the song will appear line by line, there will be an on terminal keyboard render which will emulate your keystrokes. Correct hits will be rendered in green, incorrect ones will be in red.
