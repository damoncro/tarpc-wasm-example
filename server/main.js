const express = require('express');
const app = express();
const port = 3000;

app.use((req, res, next) => {
    res.header('Access-Control-Allow-Origin', '*');
    next();
});

app.get('/', (req, res) => {
    res.send('pong ok');
});

app.get('/key_gen', (req, res) => {
    res.send('key_gen is called');
});

app.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});
