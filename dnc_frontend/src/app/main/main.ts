// src/app/layout/shell.component.ts
import {Component, Inject, inject, OnInit, PLATFORM_ID} from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router, RouterModule, NavigationEnd } from '@angular/router';

import { MatToolbarModule } from '@angular/material/toolbar';
import { MatSidenavModule } from '@angular/material/sidenav';
import {MatIconModule, MatIconRegistry} from '@angular/material/icon';
import { MatListModule } from '@angular/material/list';
import { MatButtonModule } from '@angular/material/button';
import { MatDividerModule } from '@angular/material/divider';
import { filter } from 'rxjs/operators';
import {MatLine} from '@angular/material/core';
import {LoginService, MenuActivationMap, LoggedInUser} from '../login.service';
import {MatMenu, MatMenuItem, MatMenuTrigger} from '@angular/material/menu';
import {DomSanitizer} from '@angular/platform-browser';
import {trace} from '@opentelemetry/api';

type TopNavKey = 'dashboard' | 'csr' | 'reports' | 'billing' | 'setup';

interface TopNavItem {
  key: TopNavKey;
  label: string;
  icon: string;
  route: string;
  disabled: boolean;
}

interface SideNavItem {
  label: string | {line1:string, line2:string, line3:string};
  icon: string;
  route: string;
  disabled: boolean;
}

@Component({
  selector: 'app-shell',
  standalone: true,
  imports: [
    CommonModule,
    RouterModule,
    MatToolbarModule,
    MatSidenavModule,
    MatIconModule,
    MatListModule,
    MatButtonModule,
    MatDividerModule,
    MatLine,
    MatMenu,
    MatMenuTrigger,
    MatMenuItem,
  ],
  templateUrl: './main.html',
  styleUrl: './main.scss',
})
export class MainComponent implements OnInit {
  isSidenavOpened = false;
  canSideNavToggle = true;
  activeTopNav: TopNavKey = 'dashboard';
  private router= inject(Router);
  public loginService = inject(LoginService);
  private iconRegistry = inject(MatIconRegistry);
  private sanitizer = inject(DomSanitizer);

  topNavItems: TopNavItem[] = [
    { key: 'dashboard',  label: 'Dashboard',  icon: 'dashboard',     route: '/main/dashboard',    disabled:true},
    { key: 'csr',        label: 'CSR',        icon: 'build',         route: '/main/csr',     disabled:true},
    { key: 'reports',    label: 'Reports',    icon: 'bar_chart',     route: '/main/reports', disabled:true},
    { key: 'billing',    label: 'Billing',    icon: 'receipt_long',  route: '/main/billing', disabled:true},
    { key: 'setup',      label: 'Setup',      icon: 'settings',      route: '/main/setup',   disabled:true},
  ];

  sideNavConfig: Record<TopNavKey, SideNavItem[]> = {
    dashboard: [
      { label: 'Overview',  icon: 'dashboard', route: '/home/overview', disabled:true },
      { label: 'Activity',  icon: 'timeline',  route: '/home/activity', disabled:true },
    ],
    csr: [
      { label: 'Orders',    icon: 'shopping_cart', route: '/operations/orders', disabled:true },
      { label: 'Inventory', icon: 'inventory_2',   route: '/operations/inventory',disabled: true },
      { label: 'Customers', icon: 'group',         route: '/operations/customers', disabled:true },
    ],
    reports: [
      { label: 'Sales Report',    icon: 'stacked_line_chart', route: '/reports/sales',disabled:true },
      { label: 'Performance',     icon: 'insights',           route: '/reports/performance', disabled:true },
    ],
    billing: [
      { label: 'Invoices',  icon: 'receipt',    route: '/billing/invoices', disabled:true },
      { label: 'Payments',  icon: 'payments',   route: '/billing/payments', disabled:true },
    ],
    setup: [
      { label: 'Roles and Permissions',     icon: 'security',   route: '/main/setup/roles', disabled:true },
      { label: 'Users',     icon: 'person',     route: '/main/setup/users', disabled:true},
      { label: 'Dental Services',     icon: 'info',     route: '/main/setup/dental-services', disabled:true},
      { label: 'Clinic Capabilities',     icon: 'star',     route: '/main/setup/clinic-capabilities', disabled:true },
      { label: 'HMOs',    icon: 'account_balance',       route: '/main/setup/hmos', disabled:true  },
      { label: 'Dentist Contracts',    icon: 'file_copy',       route: '/main/setup/dentist-contracts', disabled:true },
      { label: 'Dental Clinics',    icon: 'home',       route: '/main/setup/dental-clinics', disabled:true },
      { label: 'Dentists',    icon: 'face',       route: '/main/setup/dentists', disabled:true },
      { label: 'Endorsements',    icon: 'settings',       route: '/main/setup/endorsements', disabled:true },
    ],
  };
  menu_activation_map: MenuActivationMap = {};
  currentUser: LoggedInUser = {
    email: "", name: "", role_name: "", user_id: 0
  }
  user_initials : string = "";
  fullName : string = "";

  constructor(@Inject(PLATFORM_ID) private platformId: Object,) {
    this.iconRegistry.addSvgIcon('account_circle',
      this.sanitizer.bypassSecurityTrustResourceUrl('https://fonts.gstatic.com/s/i/materialicons/account_circle/v16/24px.svg'));

    // Keep activeTopNav in sync with the current URL
    this.router.events
      .pipe(filter((e) => e instanceof NavigationEnd))
      .subscribe(() => this.updateActiveTopNavFromUrl());

  }

  ngOnInit(): void {
      const tracer = trace.getTracer('manual-tracer');
      const span = tracer.startSpan('MainComponent.ngOnInit');
      console.log('Manual span started!');
      this.menu_activation_map = this.loginService?.menuActivationMap();
      this.currentUser = this.loginService?.currentUser();
      this.fullName = this.currentUser.name;
      this.user_initials = this.getInitials();
      this.configure_setup_menu();
      console.log("Manual span ended!");
  }

  getInitials(): string {
    const parts = (this.fullName ?? '')
      .trim()
      .split(/\s+/)
      .filter(Boolean);
    if (parts.length === 0) return '?';
    if (parts.length === 1) return parts[0].slice(0,2).toUpperCase();
    return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
  }
  logout(){
    this.loginService.logout();
    this.router.navigate(['/']);
  }
  configure_setup_menu(){
    if (
      "dental_service" in this.menu_activation_map ||
      "clinic capability" in this.menu_activation_map ||
      "user" in this.menu_activation_map ||
      "role" in this.menu_activation_map ||
      "hmo" in this.menu_activation_map ||
      "dental_contract" in this.menu_activation_map ||
      "clinic" in this.menu_activation_map ||
      "dentist" in this.menu_activation_map ||
      "endorsement" in this.menu_activation_map
    )
      this.topNavItems[4].disabled = false;

    this.activate_item("dental_service", "Dental Services");
    this.activate_item("clinic_capability", "Clinic Capabilities");
    this.activate_item("user", "Users");
    this.activate_item("role", "Roles and Permissions");
    this.activate_item("hmo", "HMOs");
    this.activate_item("dentist_contract", "Dentist Contracts");
    this.activate_item("dental_clinic", "Dental Clinics");
    this.activate_item("dentist", "Dentists");
    this.activate_item("endorsement", "Endorsements");
  }


  activate_item(menu_key:string, side_nav_key:string){
    const activated_item= menu_key in this.menu_activation_map;
    this.activate_SideNav(side_nav_key, activated_item);
  }

  activate_SideNav( key:string, activated:boolean |undefined ){
    const setup_sidenav = this.sideNavConfig['setup'];
    const sideNavItem = setup_sidenav.find(item => item.label === key);
    if (!sideNavItem) return;
    sideNavItem.disabled = !activated;


  }

  toggleSidenav() {
    if (this.canSideNavToggle) {
      this.isSidenavOpened = !this.isSidenavOpened;

    }
  }

  onTopNavClick(item: TopNavItem) {
    this.activeTopNav = item.key;
    this.isSidenavOpened = true;
    this.router.navigateByUrl(item.route);
  }

  get sideNavItems(): SideNavItem[] {
    return this.sideNavConfig[this.activeTopNav] ?? [];
  }

  private updateActiveTopNavFromUrl() {
    const url = this.router.url.split('?')[0].split('#')[0];
    const segments = url.split('/').filter(Boolean); // remove empty segments
    const firstSegment = (segments[1] ?? 'home') as TopNavKey;

    // If segment is not in our keys, default to 'home'
    const validKeys: TopNavKey[] = ['dashboard', 'csr', 'reports', 'billing', 'setup'];
    this.activeTopNav = validKeys.includes(firstSegment) ? firstSegment : 'dashboard';
  }
}
