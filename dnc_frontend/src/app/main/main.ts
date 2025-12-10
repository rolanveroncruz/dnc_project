// src/app/layout/shell.component.ts
import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router, RouterModule, NavigationEnd } from '@angular/router';

import { MatToolbarModule } from '@angular/material/toolbar';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatIconModule } from '@angular/material/icon';
import { MatListModule } from '@angular/material/list';
import { MatButtonModule } from '@angular/material/button';
import { MatDividerModule } from '@angular/material/divider';
import { filter } from 'rxjs/operators';

type TopNavKey = 'dashboard' | 'csr' | 'reports' | 'billing' | 'setup';

interface TopNavItem {
  key: TopNavKey;
  label: string;
  icon: string;
  route: string;
  disabled?: boolean;
}

interface SideNavItem {
  label: string;
  icon: string;
  route: string;
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
  ],
  templateUrl: './main.html',
  styleUrl: './main.scss',
})
export class MainComponent {
  isSidenavOpened = false;
  activeTopNav: TopNavKey = 'dashboard';

  topNavItems: TopNavItem[] = [
    { key: 'dashboard',  label: 'Dashboard',  icon: 'dashboard',     route: 'dashboard',    disabled:false},
    { key: 'csr',        label: 'CSR',        icon: 'build',         route: 'csr',     disabled:false},
    { key: 'reports',    label: 'Reports',    icon: 'bar_chart',     route: 'reports', disabled:false},
    { key: 'billing',    label: 'Billing',    icon: 'receipt_long',  route: 'billing', disabled:false},
    { key: 'setup',      label: 'Setup',      icon: 'settings',      route: 'setup',   disabled:false},
  ];

  sideNavConfig: Record<TopNavKey, SideNavItem[]> = {
    dashboard: [
      { label: 'Overview',  icon: 'dashboard', route: '/home/overview' },
      { label: 'Activity',  icon: 'timeline',  route: '/home/activity' },
    ],
    csr: [
      { label: 'Orders',    icon: 'shopping_cart', route: '/operations/orders' },
      { label: 'Inventory', icon: 'inventory_2',   route: '/operations/inventory' },
      { label: 'Customers', icon: 'group',         route: '/operations/customers' },
    ],
    reports: [
      { label: 'Sales Report',    icon: 'stacked_line_chart', route: '/reports/sales' },
      { label: 'Performance',     icon: 'insights',           route: '/reports/performance' },
    ],
    billing: [
      { label: 'Invoices',  icon: 'receipt',    route: '/billing/invoices' },
      { label: 'Payments',  icon: 'payments',   route: '/billing/payments' },
    ],
    setup: [
      { label: 'Dental Services',     icon: 'Dentistry',     route: '/main/setup/dental-services' },
      { label: 'Users',     icon: 'person',     route: '/setup/users' },
      { label: 'Roles',     icon: 'security',   route: '/setup/roles' },
      { label: 'System',    icon: 'tune',       route: '/setup/system' },
    ],
  };

  constructor(private router: Router) {
    // Keep activeTopNav in sync with the current URL
    this.router.events
      .pipe(filter((e) => e instanceof NavigationEnd))
      .subscribe(() => this.updateActiveTopNavFromUrl());
  }

  toggleSidenav() {
    this.isSidenavOpened = !this.isSidenavOpened;
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
    const firstSegment = (segments[0] ?? 'home') as TopNavKey;

    // If segment is not in our keys, default to 'home'
    const validKeys: TopNavKey[] = ['dashboard', 'csr', 'reports', 'billing', 'setup'];
    this.activeTopNav = validKeys.includes(firstSegment) ? firstSegment : 'dashboard';
  }
}
