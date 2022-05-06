print('Start #################################################################');
db = db.getSiblingDB('wp_users');
db.createUser(
    {
        user: 'testAdmin',
        pwd: 'w4pp13R',
        roles: [{ role: 'readWrite', db: 'wp_users' }],
    },
);
print('END #################################################################');
