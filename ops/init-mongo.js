db.createUser({
    user:process.env.MONGO_TEST_USER,
    pwd:process.env.MONGO_TEST_PASSWORD,
    roles:[{
        role:"readWrite",
        db:process.env.MONGO_INITDB_DATABASE
    }]
})