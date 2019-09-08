# actix-auth

## Possible flows

```plain
POST /auth `wrong credentials`
401 Unauthorized `bad login / password`
```

```plain
POST /auth `totp code`
401 Unauthorized `user login required first`
```

```plain
POST /auth `correct credentials`
/ 401 Unauthorized `totp required next`
\ 204 No Content `when totp disabled`
```

```plain
POST /auth `correct credentials`
401 Unauthorized `totp required next`

POST /auth `totp code`
204 No Content `user signed-in`
```
