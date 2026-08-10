import { Component, inject, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { finalize } from 'rxjs';

import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatDialog, MatDialogModule } from '@angular/material/dialog';

import {CSRDentistsService, DentistWithClinicsResponse} from './csrdentists-service';
// ✅ Adjust paths as necessary
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';

import {DentistClinicsDialogComponent} from './dentist-clinics-dialog-component/dentist-clinics-dialog-component';


/*
 * ✅ The table still contains the complete API object,
 * including clinics, but clinics are NOT displayed in
 * the main table.
 *
 * We only add dentist_name because it is convenient
 * for displaying/searching/sorting.
 */
export interface CSRDentistTableRow extends DentistWithClinicsResponse {
    dentist_name: string;
}


@Component({
    selector: 'app-csr-dentists',
    standalone: true,
    imports: [
        CommonModule,
        MatCardModule,
        MatButtonModule,
        MatIconModule,
        MatDialogModule,
        GenericDataTableComponent,
    ],
    templateUrl: './csrdentists.html',
    styleUrl: './csrdentists.scss',
})
export class CSRDentistsComponent implements OnInit {

    private readonly dentistsService = inject(CSRDentistsService);

    // ✅ Used to open the clinic detail dialog
    private readonly dialog = inject(MatDialog);


    readonly dentists = signal<CSRDentistTableRow[]>([]);
    readonly loading = signal(false);
    readonly errorMessage = signal<string | null>(null);


    /*
     * ✅ MAIN TABLE
     *
     * Only dentist information is shown here.
     * Clinics and capabilities are intentionally omitted.
     */
    readonly columns: TableColumn<CSRDentistTableRow>[] = [
        {
            key: 'dentist_name',
            label: 'Dentist',
            sortable: true,
            minWidthPx: 220,
        },
        {
            key: 'exclusive_to_companies',
            label: 'Exclusive For companies',
            cellTemplateKey: 'unorderedListCell',
            sortable: false,
            minWidthPx: 200,
        },
        {
            key: 'except_for_companies',
            label: 'Except for companies',
            cellTemplateKey: 'unorderedListCell',
            sortable: false,
            minWidthPx: 200,
        },
        {
            key: 'exclusive_to_hmos',
            label: 'Except for HMOs',
            cellTemplateKey: 'unorderedListCell',
            sortable: false,
            minWidthPx: 200,
        },
        {
            key: 'except_for_hmos',
            label: 'Except for companies',
            cellTemplateKey: 'unorderedListCell',
            sortable: false,
            minWidthPx: 200,
        },
    ];


    ngOnInit(): void {
        this.loadDentists();
    }


    // ✅ Fetch dentists and their nested clinics
    loadDentists(): void {

        this.loading.set(true);
        this.errorMessage.set(null);

        this.dentistsService
            .getDentists()
            .pipe(
                finalize(() => this.loading.set(false))
            )
            .subscribe({
                next: dentists => {

                    const rows: CSRDentistTableRow[] = dentists.map(
                        dentist => ({
                            ...dentist,

                            // ✅ Add convenient display name
                            dentist_name: this.buildDentistName(dentist),
                        })
                    );

                    this.dentists.set(rows);
                },

                error: error => {
                    console.error(
                        'Failed to load dentists:',
                        error
                    );

                    this.errorMessage.set(
                        'Unable to load dentists.'
                    );
                }
            });
    }


    /*
     * ✅ Called when GenericDataTableComponent emits rowClicked.
     */
    onDentistClicked(dentist: CSRDentistTableRow): void {

        this.dialog.open(DentistClinicsDialogComponent, {
            width: '900px',
            maxWidth: '95vw',

            data: dentist,
        });
    }


    private buildDentistName(
        dentist: DentistWithClinicsResponse
    ): string {

        const givenNames = [
            dentist.given_name,
            dentist.middle_name
        ]
            .filter(Boolean)
            .join(' ');

        return `${dentist.last_name}, ${givenNames}`;
    }
}
