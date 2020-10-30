# ukubulala-srv
REST, read-only service with USA homicide data

## Building

```bash
git clone https://github.com/yisonPylkita/ukubulala-srv
# TODO: add git lfs
cd ukubulala-srv
docker build db
docker run -it -d -p 5432:5432 -e POSTGRES_PASSWORD=docker -e POSTGRES_USER=user1 db
diesel setup
cargo run
```