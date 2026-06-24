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

export interface DentistApplicationRow {
    id: number;
    date_submitted: string | null;

    name: string;
    clinic_name: string;
    contact_numbers: string;
    email: string;

    // These are guarded backend download URLs, not raw server file paths.
    prc_license_file_path: string | null;
    bir_2303_file_path: string | null;

    status: string | null;
}

@Component({
    selector: 'app-website-applications',
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
    templateUrl: './website-applications.html',
    styleUrl: './website-applications.scss',
})
export class WebsiteApplications implements OnInit {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);
    private readonly baseUrl = environment.apiUrl;

    readonly applications = signal<DentistApplicationRow[]>([]);
    readonly isLoading = signal(false);
    readonly loadError = signal<string | null>(null);
    readonly downloadingDocumentKey = signal<string | null>(null);

    readonly displayedColumns: string[] = [
        'id',
        'date_submitted',
        'name',
        'clinic_name',
        'contact_numbers',
        'email',
        'documents',
        'status',
    ];

    ngOnInit(): void {
        this.loadApplications();
    }

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({
            Authorization: `Bearer ${token}`,
        });
    }

    loadApplications(): void {
        this.isLoading.set(true);
        this.loadError.set(null);

        this.http.get<DentistApplicationRow[]>(
            `${this.baseUrl}/api/website/dentist_applications`,
            {
                headers: this.authHeaders(),
            }
        ).subscribe({
            next: (applications) => {
                this.applications.set(applications);
                this.isLoading.set(false);
            },
            error: (error) => {
                console.error('Error loading dentist applications:', error);

                this.applications.set([]);
                this.isLoading.set(false);
                this.loadError.set(
                    'Sorry, dentist applications could not be loaded.'
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

    hasDocuments(row: DentistApplicationRow): boolean {
        return !!row.prc_license_file_path || !!row.bir_2303_file_path;
    }

    documentUrl(path: string): string {
        if (path.startsWith('http://') || path.startsWith('https://')) {
            return path;
        }

        if (path.startsWith('/')) {
            return `${this.baseUrl}${path}`;
        }

        return `${this.baseUrl}/${path}`;
    }

    downloadDocument(
        path: string,
        fallbackFileName: string,
        documentKey: string,
    ): void {
        const url = this.documentUrl(path);

        this.downloadingDocumentKey.set(documentKey);

        this.http.get(url, {
            headers: this.authHeaders(),
            responseType: 'blob',
            observe: 'response',
        }).subscribe({
            next: (response) => {
                const blob = response.body;

                this.downloadingDocumentKey.set(null);

                if (!blob) {
                    alert('The downloaded file was empty.');
                    return;
                }

                const contentDisposition = response.headers.get('content-disposition');
                const fileName = this.extractFileName(contentDisposition) ?? fallbackFileName;

                const objectUrl = window.URL.createObjectURL(blob);

                const anchor = document.createElement('a');
                anchor.href = objectUrl;
                anchor.download = fileName;
                anchor.click();

                window.URL.revokeObjectURL(objectUrl);
            },
            error: (error) => {
                console.error('Error downloading document:', error);

                this.downloadingDocumentKey.set(null);
                alert('Sorry, the document could not be downloaded.');
            },
        });
    }

    isDownloading(documentKey: string): boolean {
        return this.downloadingDocumentKey() === documentKey;
    }

    private extractFileName(contentDisposition: string | null): string | null {
        if (!contentDisposition) {
            return null;
        }

        const quotedMatch = /filename="([^"]+)"/.exec(contentDisposition);

        if (quotedMatch?.[1]) {
            return quotedMatch[1];
        }

        const plainMatch = /filename=([^;]+)/.exec(contentDisposition);

        return plainMatch?.[1]?.trim() ?? null;
    }
}
