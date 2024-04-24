const express = require('express');
const path = require('path');

// * Server Configurations
const app = express();
const port = 3000;

app.get("/", (req, res) => {
    res.json({message: "Hello, World!"});
});

app.listen(port, () => {
    console.log(`Server listening on port ${port}`);
});