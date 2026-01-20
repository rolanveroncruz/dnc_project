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
import {DentalServices} from './features/setup/dental-services/dental-services';
import {ClinicCapabilities} from './features/setup/clinic-capabilities/clinic-capabilities';
import {SetupUsers} from './features/setup/setup-users/setup-users';
import {SetupRoles} from './features/setup/setup-roles/setup-roles';
import {SetupHMOs} from './features/setup/setup-hmos/setup-hmos';
import {HMOPageComponent} from './features/setup/setup-hmos/hmopage-component/hmopage-component';
import {SetupClinics} from './features/setup/setup-clinics/setup-clinics';
import {SetupDentists} from './features/setup/setup-dentists/setup-dentists';
import {SetupEndorsements} from './features/setup/setup-endorsements/setup-endorsements';
import {authGuard} from './auth-guard';
import {SetupDentistContracts} from './features/setup/setup-dentist-contracts/setup-dentist-contracts';

export const routes: Routes = [
  {
    path:'login',
    component: LoginComponent,
    title: 'Login',
  },
  {
    path:'main',
    component: MainComponent,
    canActivate: [authGuard],
    title: 'Main',
    children: [
      { path: '', component: MainHome, title: 'Main Home'},
      {
        path: 'setup',
        title: 'Setup Home',
        children:[
          { path: '', component: SetupHome, title: 'Setup Home'},
          { path: 'dental-services', component: DentalServices, title: 'Dental Services'},
          { path: 'clinic-capabilities', component: ClinicCapabilities, title: 'Clinic Capabilities'},
          { path: 'users', component: SetupUsers, title: 'Users'},
          { path: 'roles', component: SetupRoles, title: 'Roles'},
          { path: 'hmos', component: SetupHMOs, title: 'HMOs'},
          { path: 'hmos/:id', component: HMOPageComponent , title: 'HMO'},
          { path: 'hmos/new', component: HMOPageComponent , title: 'HMO'},
          { path: 'dentist-contracts', component:SetupDentistContracts, title: 'Dental Contracts'},
          { path: 'clinics', component: SetupClinics, title: 'Clinics'},
          { path: 'dentists', component: SetupDentists, title: 'Dentists'},
          { path: 'endorsements', component: SetupEndorsements, title: 'Endorsements'},

         ]
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
  { path: '',
  component: HomeComponent,
  title: 'Home'}

];
