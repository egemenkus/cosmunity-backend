const express = require('express');
const app = express();
const cors = require('cors');
const {
  getMessagesForGroup,
  getAllGroups,
  createGroup,
  createMessage,
  getGroup,
} = require('./functions.js');
const port = 3000;
app.use(cors());
app.use(express.json());

app.get('/', async (req, res) => {
  return res.json({ message: "I'm live" });
});

app.get('/allGroups', async (req, res) => {
  return res.json(await getAllGroups());
});

app.post('/createGroup', async (req, res) => {
  const { groupName, tokenAmount } = req.body;
  const newGroup = await createGroup(groupName, tokenAmount);
  return res.json({ message: 'Group created', newGroup });
});

app.get('/getGroup/:groupId', async (req, res) => {
  const { groupId } = req.params;
  const group = await getGroup(parseInt(groupId));
  return res.json(group);
});

app.get('/getMessagesForGroup/:groupId', async (req, res) => {
  const { groupId } = req.params;
  const messages = await getMessagesForGroup(parseInt(groupId));
  return res.json(messages);
});

app.post('/createMessage', async (req, res) => {
  const { content, groupId } = req.body;
  const newMessage = await createMessage(content, parseInt(groupId));
  return res.json({ message: 'Message created', newMessage });
});

app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
