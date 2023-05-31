import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import { AuthProvider } from 'react-oidc-context';
import reportWebVitals from './reportWebVitals';
import { User } from 'oidc-client-ts';

const oidcConfig = {
  authority: process.env.REACT_APP_BS_UI_OIDC_AUTHORITY!,
  client_id: process.env.REACT_APP_BS_UI_OIDC_CLIENT_ID!,
  redirect_uri: process.env.REACT_APP_BS_UI_OIDC_REDIRECT_URI!,
  onSigninCallback: (_user: User | void): void => {
    window.history.replaceState(
      {},
      document.title,
      window.location.pathname
    )
  }
};

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
root.render(
  <React.StrictMode>
    <AuthProvider {...oidcConfig}>
      <App />
    </AuthProvider>
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
