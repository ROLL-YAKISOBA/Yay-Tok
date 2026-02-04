//import express from 'express';
//import axios from 'axios';
//import cors from'cors';
//import path from "path";

const axios = require('axios');
const express = require('express');
const cors = require('cors');
const path = require('path'); 
const app = express();

const PORT = 5000;



app.use(express.static('public'));


app.use(express.json());
app.use(cors());



app.get('/threads', (req, res) => {
    res.sendFile(path.join(__dirname, 'v2', 'threads.html'));
});


app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'v2', 'home.html'));
});

app.get('/profile', (req, res) => {
    res.sendFile(path.join(__dirname, 'public', 'profile.html'));
});

app.get('/v2/profile', (req, res) => {
    res.sendFile(path.join(__dirname, 'v2', 'profile.html'));
});


app.get('/review', (req, res) => {
    res.sendFile(path.join(__dirname, 'v2', 'review.html'));
});


app.get('/v2/review', (req, res) => {
    res.sendFile(path.join(__dirname, 'public', 'review.html'));
});ubli

app.get('/v2/threads', (req, res) => {
    res.sendFile(path.join(__dirname, 'public', 'threads.html'));
});


app.post('/proxy/threads', async (req, res) => {
   
    const { Id, token, text } = req.body;

  // console.log(req.body);

    if (!Id || !token || !text) {
        return res.status(400).json({ error: 'Missing parameters: userId, token, and text are required.' });
    }

    const targetUrl = `https://api.yay.space/v1/threads`;

    try {
        const response = await axios({
            method: 'POST',
            url: targetUrl,
            headers: {
                'Content-Type': 'application/json',
                'authorization':  `${token}`
            },
            data: {

                "group_id":`${Id}`,
                "title":`${text}`,
                "thread_icon_filename":null
               
            }
        });
        console.log(response);
      
        // Yay! からのレスポンスをそのまま返す
      
        res.status(response.status).json(response.data);

    } catch (error) {
console.error(error);
        console.error('Error forwarding request:', error.response?.data || error.message);

        // エラーの場合はその内容を返す
        res.status(error.response?.status || 500).json({
            error: 'Failed to request Yay! API',
            details: error.response?.data || error.message
        });
    }
});


app.post('/proxy/join',async(req,res)=>{


    const Id = req.body.Id;
    const token = req.body.token;

     if (!Id || !token ) {
        return res.status(400).json({ error: 'Missing parameters: groupId and token are required.' });
    };

    const targetUrl = `https://api.yay.space/v1/groups/${Id}/join`;

       try {
        const response = await axios({
            method: 'POST',
            url: targetUrl,
            headers: {
                'Content-Type': 'application/json',
                'authorization':  `${token}`
            }
            
        });
        console.log(response);
        
        res.status(response.status).json(response.data);

    } catch (error) {
           
console.error(error);
        console.error('Error forwarding request:', error.response?.data || error.message);

       
        res.status(error.response?.status || 500).json({
            error: 'Failed to request Yay! API',
            details: error.response?.data || error.message
        });
    }

    
});


app.post('/proxy/review', async (req, res) => {
    
    const { userId, token, text } = req.body;

    //  console.log(req.body);

    if (!userId || !token || !text) {
        return res.status(400).json({ error: 'Missing parameters: userId, token, and text are required.' });
    }

    const targetUrl = `https://api.yay.space/v1/users/reviews/${userId}`;

    try {
        const response = await axios({
            method: 'POST',
            url: targetUrl,
            headers: {
                'Content-Type': 'application/json',
                'authorization':  `${token}`
            },
            data: {
                comment:  `${text}`
            }
        });
        console.log(response);
        
        res.status(response.status).json(response.data);

    } catch (error) {
console.error(error);
        console.error('Error forwarding request:', error.response?.data || error.message);

       
        res.status(error.response?.status || 500).json({
            error: 'Failed to request Yay! API',
            details: error.response?.data || error.message
        });
    }
});


app.listen(PORT, '0.0.0.0', () => {
    console.log(`Server is running on http://0.0.0.0:${PORT}`);
});
