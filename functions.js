const { PrismaClient } = require('@prisma/client');
const prisma = new PrismaClient();

async function createGroup(groupName, tokenAmount) {
  const newGroup = await prisma.group.create({
    data: {
      groupName: groupName,
      tokenAmount,
    },
  });
  return newGroup;
}

async function getAllGroups() {
  return await prisma.group.findMany();
}

async function createMessage(content, groupId) {
  const newMessage = await prisma.message.create({
    data: {
      content: content,
      groupId: groupId,
    },
  });
  return newMessage;
}

async function getMessagesForGroup(groupId) {
  return await prisma.message.findMany({
    where: {
      groupId: groupId,
    },
  });
}

async function createMessage(content, groupId) {
  const newMessage = await prisma.message.create({
    data: {
      content: content,
      groupId: groupId,
    },
  });
  return newMessage;
}

async function getGroup(groupId) {
  return await prisma.group.findUnique({
    where: {
      id: groupId,
    },
  });
}

module.exports = {
  createGroup,
  getAllGroups,
  createMessage,
  getMessagesForGroup,
  createMessage,
  getGroup,
};
