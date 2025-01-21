import React, { useState, useEffect } from 'react';
import axios from 'axios';
//https://react-bootstrap.netlify.app/docs/components/buttons
import 'bootstrap/dist/css/bootstrap.min.css';
import Button from 'react-bootstrap/Button';

import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';


const App = () => {
  const [tasks, setTasks] = useState([]);
  const [newTask, setNewTask] = useState("");

  useEffect(() => {
    fetchTasks();
  }, []);

  const fetchTasks = async () => {
    try {
      const response = await axios.get('http://localhost:8000/tasks');
      setTasks(response.data);
    } catch (error) {
      console.error("Error fetching tasks", error);
    }
  };

  const addTask = async () => {
    try {
      const response = await axios.post('http://localhost:8000/tasks', { text: newTask });
      setTasks([...tasks, response.data]);
      setNewTask("");
    } catch (error) {
      console.error("Error adding task", error);
    }
  };

  const deleteTask = async (id) => {
    try {
      await axios.delete(`http://localhost:8000/tasks/${id}`);
      setTasks(tasks.filter(task => task.id !== id));
    } catch (error) {
      console.error("Error deleting task", error);
    }
  };

  return (
    <div>
      <Container>
        <Row>
          <Col>
            <h1>To-Do List</h1>
          </Col>

        </Row>
        <Row>
          <Col>
            <input
              type="text"
              value={newTask}
              onChange={(e) => setNewTask(e.target.value)}
              placeholder="Add a new task"
            />
          </Col>
          <Col><Button variant="info" onClick={addTask}>Add</Button></Col>
        </Row>
        {tasks.map((task) => (
          <Row>
            <Col>{task.text}</Col>
            <Col>
              <Button variant="primary" onClick={() => deleteTask(task.id)}>Delete</Button>
            </Col>
          </Row>
        ))}
      </Container>
    </div>
  );
};

export default App;
