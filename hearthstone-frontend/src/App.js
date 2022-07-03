import 'bootstrap/dist/css/bootstrap.min.css';
import './App.css';
import NavigationSystem from './NavigationSystem.js';
import AxiosInstanceProvider from './utility/AxiosContextProvider';

function App() {
  return (
    <AxiosInstanceProvider config={{ baseURL: "/" }}>
      <div containter="App">
        <NavigationSystem />
      </div>
    </AxiosInstanceProvider>
  );
}

export default App;
