```rust
User = { _id: ObjectId # ユーザの一意のID
      name: String # ユーザ名
      email: String # メールアドレス
      password: String # ハッシュ化する
    }
Post{
      _id: ObjectId # ユーザの一意のID
      user: db.user.collection._id
      title: String # タイトル
      content: String # 本文
      }
```
