# Aptoma Smooth Storage CLI

A small CLI tool for working with Aptoma Smooth Storage.


## Example Usage

```console
$ ass-cli -a account-name image upload data/image.jpg
Image uploaded: {
  "author": null,
  "created": "2019-01-19T14:32:19.000Z",
  "description": null,
  "height": 1200,
  "id": 1205771,
  "md5": "8afcec7eca2329d924b9aa112adc568b",
  "name": "image.jpg",
  "original_url": "...",
  "size": 545244,
  "source_url": null,
  "title": null,
  "updated": "2019-01-19T14:32:19.000Z",
  "user_id": 2,
  "width": 1600
}
URL: https://smooth-storage-url.com/users/account-name/images/1205771.jpg?accessToken=407c8eefb5cd5bcd8ef00f243cc467c367da68336af3ef13a28ce4ea28a46d85
```

For further usage see
```console
$ ass-cli --help
```

## Account files

Account configuration files are stored as `.json` files with the following format:
```json
{
  "url": "https://smooth-storage-url.com",
  "name": "account-name",
  "apikey": "account api key"
}
```

Create one configuration file for each account you want to use, and save them in `~/.config/ass-cli` (or similar default configuration path for non-Linux systems). When invoking `ass-cli` you can specify which configuration file to use by supplying the filename, without `.json` post fix, to the `-a/--account` parameter. So, if you have a file `~/.config/ass-cli/dev.json`, you can specify that account using `ass-cli -a dev <ASS CLI COMMAND>`.

When invoking `ass-cli` you can also specify the path to this configuration file using the `-c/--config` parameter.
