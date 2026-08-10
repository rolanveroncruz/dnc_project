import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';

import {
    MAT_DIALOG_DATA,
    MatDialogModule,
    MatDialogRef
} from '@angular/material/dialog';

import { MatTableModule } from '@angular/material/table';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';

import {DentistClinicResponse, DentistWithClinicsResponse} from '../csrdentists-service';

/*
 * ✅ View model used only by the dialog table.
 */
interface DentistClinicTableRow {
    id: number;
    name: string;
    address: string;
    capabilities: string;
    contact_number: string;
    schedule: string;
}


@Component({
    selector: 'app-dentist-clinics-dialog',
    standalone: true,
    imports: [
        CommonModule,
        MatDialogModule,
        MatTableModule,
        MatButtonModule,
        MatIconModule,
    ],
    templateUrl: './dentist-clinics-dialog-component.html',
    styleUrl: './dentist-clinics-dialog-component.scss',
})
export class DentistClinicsDialogComponent {

    readonly data =
        inject<DentistWithClinicsResponse>(MAT_DIALOG_DATA);

    private readonly dialogRef =
        inject(MatDialogRef<DentistClinicsDialogComponent>);


    // ✅ Columns in the clinic table
    readonly displayedColumns: string[] = [
        'name',
        'address',
        'capabilities',
        'contact_number',
        'schedule',
    ];


    /*
     * ✅ Convert the nested API response into table rows.
     *
     * Capabilities become:
     *
     * "X-Ray, Oral Surgery, Orthodontics"
     */
    readonly clinics: DentistClinicTableRow[] =
        this.data.clinics.map(
            clinic => this.toTableRow(clinic)
        );


    get dentistName(): string {

        const givenNames = [
            this.data.given_name,
            this.data.middle_name
        ]
            .filter(Boolean)
            .join(' ');

        return `${this.data.last_name}, ${givenNames}`;
    }


    close(): void {
        this.dialogRef.close();
    }


    private toTableRow(
        clinic: DentistClinicResponse
    ): DentistClinicTableRow {

        let data ={
            id: clinic.id,
            name: clinic.name,
            address: clinic.address,

            // ✅ Convert capabilities into comma-separated text
            capabilities: clinic.capabilities
                .map(capability => capability.name)
                .join(', '),
            contact_number: clinic.contact_number,
            schedule: clinic.schedule,
        };
        console.log('data', data);
        return data;
    }
}
