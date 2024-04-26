require('dotenv').config();
const path = require('path');
const express = require('express');

// TODO Need to integrate Vue to the back end to serve it from this
// See if it can be hosted on a raspberry Pi...
// It is currently committed there and pulled down but how do you run this??

// * Server Configurations
const app = express();
const { PORT } = process.env;

app.use(express.static(path.join(__dirname, 'public')));

app.get("/", (req, res) => {
    res.sendFile(path.join(__dirname, 'public/index.html'));
});

app.listen(PORT, () => {
    console.log(`Server listening on port ${PORT}`);
}); 