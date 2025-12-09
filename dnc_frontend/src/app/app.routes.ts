import { Routes } from '@angular/router';
import {Login} from './login/login';
import {Home} from './home/home';
import {Main} from './main/main';

export const routes: Routes = [
  {
    path:'login',
    component: Login,
    title: 'Login',
  },
  {
    path:'',
    component: Home,
    title: 'Home',
  },
  {
    path:'main',
    component: Main,
    title: 'Main',
  },

];
