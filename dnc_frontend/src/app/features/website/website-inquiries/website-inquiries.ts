import {Component, inject, OnInit, signal} from '@angular/core';
import {CommonModule} from '@angular/common';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {MatCard} from '@angular/material/card';
import {MatButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {MatProgressSpinner} from '@angular/material/progress-spinner';
import {
    MatTable,
    MatColumnDef,
    MatHeaderCell,
    MatHeaderCellDef,
    MatCell,
    MatCellDef,
    MatHeaderRow,
    MatHeaderRowDef,
    MatRow,
    MatRowDef,
} from '@angular/material/table';

import {environment} from '../../../../environments/environment';
import {LoginService} from '../../../login.service';

export interface ContactUsMessageRow {
    id: number;
    date_submitted: string;

    person_type: string;
    name: string;
    card_number: string | null;
    company_and_hmo: string | null;
    contact_numbers: string;
    message: string;

    status: string;
}

@Component({
    selector: 'app-website-inquiries',
    imports: [
        CommonModule,
        MatCard,
        MatButton,
        MatIcon,
        MatProgressSpinner,

        MatTable,
        MatColumnDef,
        MatHeaderCell,
        MatHeaderCellDef,
        MatCell,
        MatCellDef,
        MatHeaderRow,
        MatHeaderRowDef,
        MatRow,
        MatRowDef,
    ],
    templateUrl: './website-inquiries.html',
    styleUrl: './website-inquiries.scss',
})
export class WebsiteInquiries implements OnInit {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);
    private readonly baseUrl = environment.apiUrl;

    readonly inquiries = signal<ContactUsMessageRow[]>([]);
    readonly isLoading = signal(false);
    readonly loadError = signal<string | null>(null);

    readonly displayedColumns: string[] = [
        'id',
        'date_submitted',
        'person_type',
        'name',
        'member_info',
        'contact_numbers',
        'message',
        'status',
    ];

    ngOnInit(): void {
        this.loadInquiries();
    }

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';

        return new HttpHeaders({
            Authorization: `Bearer ${token}`,
        });
    }

    loadInquiries(): void {
        this.isLoading.set(true);
        this.loadError.set(null);

        this.http.get<ContactUsMessageRow[]>(
            `${this.baseUrl}/api/website/contact_us_messages`,
            {
                headers: this.authHeaders(),
            }
        ).subscribe({
            next: (inquiries) => {
                this.inquiries.set(inquiries);
                this.isLoading.set(false);
            },
            error: (error) => {
                console.error('Error loading contact us messages:', error);

                this.inquiries.set([]);
                this.isLoading.set(false);
                this.loadError.set(
                    'Sorry, website inquiries could not be loaded.'
                );
            },
        });
    }

    formatDate(value: string | null): string {
        if (!value) {
            return '—';
        }

        const date = new Date(value);

        if (Number.isNaN(date.getTime())) {
            return value;
        }

        return date.toLocaleString();
    }

    displayValue(value: string | null | undefined): string {
        const cleaned = value?.trim();

        return cleaned ? cleaned : '—';
    }

    displayPersonType(value: string): string {
        switch (value) {
            case 'member':
                return 'Member';
            case 'dentist':
                return 'Dentist';
            case 'broker':
                return 'Broker';
            case 'hmo_rep':
                return 'HMO Representative';
            default:
                return this.displayValue(value);
        }
    }

    hasMemberInfo(row: ContactUsMessageRow): boolean {
        return !!row.card_number || !!row.company_and_hmo;
    }
}
