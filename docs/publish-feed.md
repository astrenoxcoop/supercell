# How to publish a feed

Supercell is a feed generator, that is to say it is the service that fulfills feed requests and returns the posts that are displayed.

To invoke Supercell, you first need to create a record on your PDS saying that you are publishing a feed for others to use. This is called feed publishing.

This is done by creating a record on your PDS with the following structure:

```json
{
  "$type": "app.bsky.feed.generator",
  "did": "did:web:the_hostname",
  "displayName": "Feed A",
  "description": "A useful feed.",
  "createdAt": "2024-10-30T16:15:31Z"
}
```

The `did` is a did:web identifier where the hostname is used to service a DID document that contains a "BskyFeedGenerator" service structure with the hostname to make API calls.

## Publish script

The `publish.py` script can be used to create new feed records or update existing ones.

1. Create a new virtual environment to run the script in: `python -m venv ./venv/`
2. Install the atproto library in the virtual environment: `./venv/bin/pip install atproto`
3. Invoke the script using the virtual environment: `./venv/bin/python ./etc/publish.py --help`
