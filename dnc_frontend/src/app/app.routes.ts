import { Routes } from '@angular/router';
import {LoginComponent} from './login/login';
import {HomeComponent} from './home/home';
import {MainComponent} from './main/main';
import {SetupHome} from './features/setup/setup-home/setup-home';
import {DashboardHome} from './features/dashboard/dashboard-home/dashboard-home';
import {CsrHome} from './features/csr/csr-home/csr-home';
import {ReportsHome} from './features/reports/reports-home/reports-home';
import {BillingHome} from './features/billing/billing-home/billing-home';
import {MainHome} from './features/main/main-home/main-home';

export const routes: Routes = [
  {
    path:'login',
    component: LoginComponent,
    title: 'Login',
  },
  {
    path:'main',
    component: MainComponent,
    title: 'Main',
    children: [
      { path: '', component: MainHome, title: 'Main Home'},
      {
        path: 'setup',
        component: SetupHome,
        title: 'Setup Home',
      }, // end of 'setup'
      {
        path:'dashboard',
        component: DashboardHome,
        title: 'Dashboard Home',
      }, // end of 'dashboard'
      {
        path:'csr',
        component: CsrHome,
        title: 'CSR Home',
      }, // end of 'csr'
      {
        path: 'reports',
        component: ReportsHome,
        title: 'Reports Home',
      }, // end of 'reports'
      {
        path: 'billing',
        component: BillingHome,
        title: 'Billing Home',
      } // end of 'billing'

    ]
  },

];
