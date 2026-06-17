// src/app/layout/shell.component.ts
import { Component, Inject, inject, OnInit, PLATFORM_ID } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router, RouterModule, NavigationEnd } from '@angular/router';

import { MatToolbarModule } from '@angular/material/toolbar';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatIconModule, MatIconRegistry } from '@angular/material/icon';
import { MatListModule } from '@angular/material/list';
import { MatButtonModule } from '@angular/material/button';
import { MatDividerModule } from '@angular/material/divider';
import { MatExpansionModule } from '@angular/material/expansion';

import { filter } from 'rxjs/operators';
import { MatLine } from '@angular/material/core';
import { LoginService, MenuActivationMap, LoggedInUser } from '../login.service';
import { MatMenu, MatMenuItem, MatMenuTrigger } from '@angular/material/menu';
import { DomSanitizer } from '@angular/platform-browser';
import { trace } from '@opentelemetry/api';

type TopNavKey = 'dashboard' | 'csr' | 'reports' | 'billing' | 'setup';

interface TopNavItem {
    key: TopNavKey;
    label: string;
    icon: string;
    route: string;
    disabled: boolean;
}

interface SideNavItem {
    label: string | { line1: string; line2: string; line3: string };
    icon: string;
    route?: string; // ✅ optional because parent submenu items may not navigate directly
    disabled: boolean;
    children?: SideNavItem[]; // ✅ supports second-level submenu items
}

/*
The MainComponent is the main layout component that contains the top navigation, side navigation, and main content area.
When a user logs in, login returns a menuActivationMap object that contains the permissions of the user.
The MenuActivationMap is a structure defined in the LoginService as: Record<string, string>. The former string is the dataobject,
while the latter string is "enabled".
Inside ngOnInit() are function_calls: this.configure_X_menu (where X is the menu item). This function then selectively enables or disables
the TopNavItem and the SideNavItems.
*/
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
        MatExpansionModule, // ✅ needed for mat-expansion-panel

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

    private router = inject(Router);
    public loginService = inject(LoginService);
    private iconRegistry = inject(MatIconRegistry);
    private sanitizer = inject(DomSanitizer);

    topNavItems: TopNavItem[] = [
        { key: 'dashboard', label: 'Dashboard', icon: 'dashboard', route: '/main/dashboard', disabled: true },
        { key: 'csr', label: 'CSR', icon: 'build', route: '/main/csr', disabled: true },
        { key: 'reports', label: 'Reports', icon: 'bar_chart', route: '/main/reports', disabled: true },
        { key: 'billing', label: 'Billing', icon: 'receipt_long', route: '/main/billing', disabled: true },
        { key: 'setup', label: 'Setup', icon: 'settings', route: '/main/setup', disabled: true },
    ];

    sideNavConfig: Record<TopNavKey, SideNavItem[]> = {
        dashboard: [
            { label: 'Operations', icon: 'dashboard', route: '/main/dashboard/operations', disabled: true },
            { label: 'Outliers', icon: 'timeline', route: '/main/dashboard/outliers', disabled: true },
        ],

        csr: [
            { label: 'Verifications', icon: 'shopping_cart', route: '/main/csr/verifications', disabled: true },
            { label: 'Inventory', icon: 'inventory_2', route: '/operations/inventory', disabled: true },
            { label: 'Customers', icon: 'group', route: '/operations/customers', disabled: true },
            { label: 'HighEndVerification', icon: 'group', route: '/main/csr/high_end_verification', disabled: true },
        ],

        reports: [
            { label: 'Sales Report', icon: 'stacked_line_chart', route: '/reports/sales', disabled: true },
            { label: 'Performance', icon: 'insights', route: '/reports/performance', disabled: true },
        ],

        billing: [
            {
                label: 'Accomplishment Reporting',
                icon: 'receipt',
                route: '/main/billing/acc_recon',
                disabled: true,
            },
            {
                label: 'HMO Billing',
                icon: 'receipt',
                route: '/main/billing/hmo_billing',
                disabled: true,
                children: [
                    {
                        label: 'Utilization Reports',
                        icon: 'add_card',
                        route: '/main/billing/hmo_billing/utils',
                        disabled: true,
                    },
                    {
                        label: 'Billing Statements',
                        icon: 'add_card',
                        route: '/main/billing/hmo_billing/statements',
                        disabled: true,
                    },
                ],
            },
            {
                label: 'Dentist Payments',
                icon: 'payments',
                route: '/main/billing/dentists',
                disabled: true,
                children: [
                    {
                        label: 'Monthly Services Counts',
                        icon: 'history',
                        route: '/main/billing/dentists/monthly_services_counts',
                        disabled: true,

                    },
                    {
                        label: 'Retainer Fees Paid',
                        icon: 'history',
                        route: '/main/billing/dentists/paid_retainer_fees',
                        disabled: true,

                    },
                    {
                        label: 'Retainer Fee Reports',
                        icon: 'history',
                        route: '/main/billing/dentists/retainer_fees',
                        disabled: true,
                    },
                    {
                        label: 'Summary of Claims',
                        icon: 'history',
                        route: '/main/billing/dentists/claims_matrix',
                        disabled: true,
                    },
                ],
            },
        ],

        setup: [
            { label: 'Roles and Permissions', icon: 'security', route: '/main/setup/roles', disabled: true },
            { label: 'Users', icon: 'person', route: '/main/setup/users', disabled: true },
            { label: 'Dental Services', icon: 'info', route: '/main/setup/dental-services', disabled: true },
            { label: 'Clinic Capabilities', icon: 'star', route: '/main/setup/clinic-capabilities', disabled: true },
            { label: 'HMOs', icon: 'account_balance', route: '/main/setup/hmos', disabled: true },
            { label: 'Dentist Contracts', icon: 'file_copy', route: '/main/setup/dentist-contracts', disabled: true },
            { label: 'Dental Clinics', icon: 'home', route: '/main/setup/dental-clinics', disabled: true },
            { label: 'Dentists', icon: 'face', route: '/main/setup/dentists', disabled: true },
            { label: 'Endorsements', icon: 'settings', route: '/main/setup/endorsements', disabled: true },
        ],
    };

    menu_activation_map: MenuActivationMap = {};

    currentUser: LoggedInUser = {
        email: '',
        name: '',
        role_name: '',
        user_id: 0,
    };

    user_initials = '';
    fullName = '';

    constructor(@Inject(PLATFORM_ID) private platformId: Object) {
        this.iconRegistry.addSvgIcon(
            'account_circle',
            this.sanitizer.bypassSecurityTrustResourceUrl(
                'https://fonts.gstatic.com/s/i/materialicons/account_circle/v16/24px.svg'
            )
        );

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
        this.configure_csr_menu();
        this.configure_billing_menu();
        this.configure_dashboard_menu();


        span.end(); // ✅ actually end the span
        console.log('Manual span ended!');
    }

    getInitials(): string {
        const parts = (this.fullName ?? '')
            .trim()
            .split(/\s+/)
            .filter(Boolean);

        if (parts.length === 0) return '?';
        if (parts.length === 1) return parts[0].slice(0, 2).toUpperCase();

        return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
    }

    logout() {
        this.loginService.logout();
        this.router.navigate(['/']);
    }

    configure_setup_menu() {
        if (
            'dental_service' in this.menu_activation_map ||
            'clinic_capability' in this.menu_activation_map ||
            'user' in this.menu_activation_map ||
            'role' in this.menu_activation_map ||
            'hmo' in this.menu_activation_map ||
            'dentist_contract' in this.menu_activation_map ||
            'dental_clinic' in this.menu_activation_map ||
            'dentist' in this.menu_activation_map ||
            'endorsements' in this.menu_activation_map
        ) {
            this.topNavItems[4].disabled = false;

            this.activate_item('setup', 'dental_service', 'Dental Services');
            this.activate_item('setup', 'clinic_capability', 'Clinic Capabilities');
            this.activate_item('setup', 'user', 'Users');
            this.activate_item('setup', 'role', 'Roles and Permissions');
            this.activate_item('setup', 'hmo', 'HMOs');
            this.activate_item('setup', 'dentist_contract', 'Dentist Contracts');
            this.activate_item('setup', 'dental_clinic', 'Dental Clinics');
            this.activate_item('setup', 'dentist', 'Dentists');
            this.activate_item('setup', 'endorsements', 'Endorsements');
        }
    }

    configure_csr_menu() {
        if ('verifications' in this.menu_activation_map) {
            this.topNavItems[1].disabled = false;
            this.activate_item('csr', 'verifications', 'Verifications');
        }

        if ('high_end_verification_information' in this.menu_activation_map) {
            this.topNavItems[1].disabled = false;
            this.activate_item('csr', 'high_end_verification_information', 'HighEndVerification');
        }
    }

    configure_billing_menu() {
        if ('acc_reconciliation' in this.menu_activation_map) {
            this.topNavItems[3].disabled = false;
            this.activate_SideNav('billing', 'Accomplishment Reporting', true);
            this.activate_SideNav('billing', 'HMO Billing', true);
            this.activate_SideNav('billing', 'Utilization Reports', true);
            this.activate_SideNav('billing', 'Billing Statements', true);
            this.activate_SideNav('billing', 'Dentist Payments', true);
            this.activate_SideNav('billing', 'Claims to HMOs', true);
            this.activate_SideNav('billing', 'Services Performed', true);
            this.activate_SideNav('billing', 'Summary of Claims', true);
            this.activate_SideNav('billing', 'Retainer Fee Reports', true);
            this.activate_SideNav('billing', 'Monthly Services Counts', true);
            this.activate_SideNav('billing', 'Retainer Fees Paid', true);
        }

        // ✅ HMO Billing submenu permissions
        if ('hmo_billing' in this.menu_activation_map) {
            this.topNavItems[3].disabled = false;
        }

        if ('utilization_reports' in this.menu_activation_map) {
            this.topNavItems[3].disabled = false;
        }

        if ('billing_statements' in this.menu_activation_map) {
            this.topNavItems[3].disabled = false;
        }

        // ✅ Dentist Payments submenu permissions
        if ('dentist_payments' in this.menu_activation_map) {
            this.topNavItems[3].disabled = false;
        }

        if ('summary_of_claims' in this.menu_activation_map) {
            this.topNavItems[3].disabled = false;
        }

        if ('retainer_fee_reports' in this.menu_activation_map) {
            this.topNavItems[3].disabled = false;
        }

        // ✅ If any child is enabled, enable the parent submenu automatically
        this.enableParentIfAnyChildEnabled('billing', 'HMO Billing');
        this.enableParentIfAnyChildEnabled('billing', 'Dentist Payments');
    }

    configure_dashboard_menu(){
        if ('dashboard' in this.menu_activation_map) {
            this.topNavItems[0].disabled = false;
            this.activate_item('dashboard', 'dashboard', 'Operations');
            this.activate_item('dashboard', 'dashboard', 'Outliers');
        }
    }

    activate_item(topnav_item: TopNavKey, menu_key: string, side_nav_key: string) {
        const activated_item = menu_key in this.menu_activation_map;
        this.activate_SideNav(topnav_item, side_nav_key, activated_item);
    }

    activate_SideNav(side_nav_key: TopNavKey, key: string, activated: boolean | undefined) {
        const items = this.sideNavConfig[side_nav_key];

        const sideNavItem = this.findSideNavItem(items, key);

        if (!sideNavItem) return;

        sideNavItem.disabled = !activated;
    }

    private findSideNavItem(items: SideNavItem[], key: string): SideNavItem | undefined {
        for (const item of items) {
            if (item.label === key) {
                return item;
            }

            if (item.children?.length) {
                const found = this.findSideNavItem(item.children, key);
                if (found) {
                    return found;
                }
            }
        }

        return undefined;
    }

    private enableParentIfAnyChildEnabled(topnav_item: TopNavKey, parent_label: string) {
        const parent = this.findSideNavItem(this.sideNavConfig[topnav_item], parent_label);

        if (!parent?.children?.length) return;

        const hasEnabledChild = parent.children.some((child) => !child.disabled);

        if (hasEnabledChild) {
            parent.disabled = false;
        }
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
        const segments = url.split('/').filter(Boolean);

        const firstSegment = (segments[1] ?? 'dashboard') as TopNavKey;

        const validKeys: TopNavKey[] = ['dashboard', 'csr', 'reports', 'billing', 'setup'];

        this.activeTopNav = validKeys.includes(firstSegment)
            ? firstSegment
            : 'dashboard';
    }
}
