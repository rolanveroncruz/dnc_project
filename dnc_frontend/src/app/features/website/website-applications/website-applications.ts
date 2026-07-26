import {Component, inject, OnInit, signal} from '@angular/core';
import {CommonModule} from '@angular/common';
import {finalize} from 'rxjs';

import {MatCardModule} from '@angular/material/card';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatSelectModule} from '@angular/material/select';

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

import {
    DentistApplicationRow,
    DentistApplicationStatus,
    WebsiteApplicationsService,
} from './website-applications-service';

@Component({
    selector: 'app-website-applications',
    imports: [
        CommonModule,

        MatCardModule,
        MatButtonModule,
        MatIconModule,
        MatProgressSpinnerModule,
        MatFormFieldModule,
        MatSelectModule,

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
    private readonly applicationsService = inject(WebsiteApplicationsService);

    readonly applications = signal<DentistApplicationRow[]>([]);
    readonly isLoading = signal(false);
    readonly loadError = signal<string | null>(null);
    readonly downloadingDocumentKey = signal<string | null>(null);

    /*
     * A Set allows separate rows to have their own updating state.
     * This also makes the implementation safe if status updates are
     * later triggered from somewhere other than the dropdown.
     */
    readonly updatingStatusIds = signal<Set<number>>(new Set());

    readonly statusOptions: ReadonlyArray<{
        value: DentistApplicationStatus;
        label: string;
    }> = [
        {
            value: 'new',
            label: 'New',
        },
        {
            value: 'for_evaluation',
            label: 'For evaluation',
        },
        {
            value: 'declined',
            label: 'Declined',
        },
        {
            value: 'accredited',
            label: 'Accredited',
        },
    ];

    readonly displayedColumns: string[] = [
        'id',
        'date_submitted',
        'name',
        'clinic_name',
        'clinic_ownership_type',
        'clinic_address',
        'hmo_affiliations',
        'contact_numbers',
        'email',
        'documents',
        'status',
    ];

    ngOnInit(): void {
        this.loadApplications();
    }

    loadApplications(): void {
        this.isLoading.set(true);
        this.loadError.set(null);

        this.applicationsService.getApplications()
            .pipe(
                finalize(() => {
                    this.isLoading.set(false);
                }),
            )
            .subscribe({
                next: (applications) => {
                    this.applications.set(
                        applications.map(application => ({
                            ...application,
                            status: this.normalizeStatus(application.status),
                        })),
                    );
                },
                error: (error) => {
                    console.error(
                        'Error loading dentist applications:',
                        error,
                    );

                    this.applications.set([]);
                    this.loadError.set(
                        'Sorry, dentist applications could not be loaded.',
                    );
                },
            });
    }

    onStatusChange(
        row: DentistApplicationRow,
        newStatus: DentistApplicationStatus,
    ): void {
        const previousStatus = this.normalizeStatus(row.status);

        if (previousStatus === newStatus) {
            return;
        }

        /*
         * Optimistically update the displayed row. If the request fails,
         * the previous value is restored.
         */
        this.replaceApplicationStatus(row.id, newStatus);
        this.setStatusUpdating(row.id, true);

        this.applicationsService.updateStatus(row.id, newStatus)
            .pipe(
                finalize(() => {
                    this.setStatusUpdating(row.id, false);
                }),
            )
            .subscribe({
                next: (response) => {
                    this.replaceApplicationStatus(
                        row.id,
                        this.normalizeStatus(response.status),
                    );
                },
                error: (error) => {
                    console.error(
                        'Error updating dentist application status:',
                        error,
                    );

                    this.replaceApplicationStatus(
                        row.id,
                        previousStatus,
                    );

                    alert(
                        'Sorry, the application status could not be updated.',
                    );
                },
            });
    }

    isUpdatingStatus(applicationId: number): boolean {
        return this.updatingStatusIds().has(applicationId);
    }

    statusValue(
        value: string | null | undefined,
    ): DentistApplicationStatus {
        return this.normalizeStatus(value);
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
        return !!row.prc_license_file_path ||
            !!row.bir_2303_file_path ||
            !!row.registration_doc_file_path ||
            !!row.supporting_docs_file_path1;
    }

    downloadDocument(
        path: string,
        fallbackFileName: string,
        documentKey: string,
    ): void {
        this.downloadingDocumentKey.set(documentKey);

        this.applicationsService.downloadDocument(path)
            .pipe(
                finalize(() => {
                    this.downloadingDocumentKey.set(null);
                }),
            )
            .subscribe({
                next: (response) => {
                    const blob = response.body;

                    if (!blob) {
                        alert('The downloaded file was empty.');
                        return;
                    }

                    const contentDisposition =
                        response.headers.get('content-disposition');

                    const fileName =
                        this.extractFileName(contentDisposition) ??
                        fallbackFileName;

                    const objectUrl = window.URL.createObjectURL(blob);

                    const anchor = document.createElement('a');
                    anchor.href = objectUrl;
                    anchor.download = fileName;
                    anchor.click();

                    window.URL.revokeObjectURL(objectUrl);
                },
                error: (error) => {
                    console.error(
                        'Error downloading document:',
                        error,
                    );

                    alert(
                        'Sorry, the document could not be downloaded.',
                    );
                },
            });
    }

    isDownloading(documentKey: string): boolean {
        return this.downloadingDocumentKey() === documentKey;
    }

    formatOwnershipType(
        value: string | null | undefined,
    ): string {
        switch (value) {
            case 'single_proprietorship':
                return 'Single Proprietorship';

            case 'company':
                return 'Company';

            case 'corporation':
                return 'Corporation';

            default:
                return this.displayValue(value);
        }
    }

    registrationDocumentLabel(
        row: DentistApplicationRow,
    ): string {
        if (row.clinic_ownership_type === 'single_proprietorship') {
            return 'DTI Registration';
        }

        if (
            row.clinic_ownership_type === 'company' ||
            row.clinic_ownership_type === 'corporation'
        ) {
            return 'SEC Registration';
        }

        return 'Registration';
    }

    registrationFallbackFileName(
        row: DentistApplicationRow,
    ): string {
        if (row.clinic_ownership_type === 'single_proprietorship') {
            return 'dti-registration';
        }

        if (
            row.clinic_ownership_type === 'company' ||
            row.clinic_ownership_type === 'corporation'
        ) {
            return 'sec-registration';
        }

        return 'registration-document';
    }

    private normalizeStatus(
        value: string | null | undefined,
    ): DentistApplicationStatus {
        const normalized = value
            ?.trim()
            .toLowerCase()
            .replace(/[\s-]+/g, '_');

        switch (normalized) {
            case 'for_evaluation':
                return 'for_evaluation';

            case 'declined':
                return 'declined';

            case 'accredited':
                return 'accredited';

            case 'new':
            default:
                return 'new';
        }
    }

    private replaceApplicationStatus(
        applicationId: number,
        status: DentistApplicationStatus,
    ): void {
        this.applications.update(applications =>
            applications.map(application =>
                application.id === applicationId
                    ? {
                        ...application,
                        status,
                    }
                    : application,
            ),
        );
    }

    private setStatusUpdating(
        applicationId: number,
        updating: boolean,
    ): void {
        this.updatingStatusIds.update(currentIds => {
            const updatedIds = new Set(currentIds);

            if (updating) {
                updatedIds.add(applicationId);
            } else {
                updatedIds.delete(applicationId);
            }

            return updatedIds;
        });
    }

    private extractFileName(
        contentDisposition: string | null,
    ): string | null {
        if (!contentDisposition) {
            return null;
        }

        /*
         * Prefer filename*= because it supports UTF-8 and encoded names.
         */
        const encodedMatch =
            /filename\*=UTF-8''([^;]+)/i.exec(contentDisposition);

        if (encodedMatch?.[1]) {
            try {
                return decodeURIComponent(encodedMatch[1]);
            } catch {
                return encodedMatch[1];
            }
        }

        const quotedMatch =
            /filename="([^"]+)"/i.exec(contentDisposition);

        if (quotedMatch?.[1]) {
            return quotedMatch[1];
        }

        const plainMatch =
            /filename=([^;]+)/i.exec(contentDisposition);

        return plainMatch?.[1]?.trim() ?? null;
    }
}
