import React from 'react';
import { Button, Space } from 'antd';

const App: React.FC = () => (
  <Space wrap>
    <Button type="primary">Primary Button</Button>
    <Button>Default Button</Button>
    <Button type="dashed">Dashed Button</Button>
    <Button type="text">Text Button</Button>
    <Button type="link">Link Button</Button>
  </Space>
);

export default App;

// Import the functions you need from the SDKs you need
import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration
// For Firebase JS SDK v7.20.0 and later, measurementId is optional
const firebaseConfig = {
  apiKey: "AIzaSyDKcOO0bppK4cjR5fQtKoMVuxI5WxbdBNI",
  authDomain: "ai-matcher.firebaseapp.com",
  projectId: "ai-matcher",
  storageBucket: "ai-matcher.appspot.com",
  messagingSenderId: "477262247110",
  appId: "1:477262247110:web:3ca926c6939bddefbb53c7",
  measurementId: "G-TWPM8Z7VQP"
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);
const analytics = getAnalytics(app);