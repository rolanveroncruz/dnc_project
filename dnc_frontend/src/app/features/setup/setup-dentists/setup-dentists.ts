import {Component, DestroyRef, inject, signal} from '@angular/core';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {Router} from '@angular/router';
import {DentistService, DentistWithLookups} from '../../../api_services/dentist-service';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {DentalClinicRow} from '../setup-dental-clinics-component/setup-dental-clinics-component';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {MatButton} from '@angular/material/button';
import {DentalClinicService} from '../../../api_services/dental-clinic-service';
import {DentistClinicService, DentistClinicWithNames} from '../../../api_services/dentist-clinic-service';
import {forkJoin} from 'rxjs';

export interface DentistWithLookupsAndClinicInfo extends DentistWithLookups {
    clinic_name: string | null;
    position: string | null;
    schedule: string | null;
}

@Component({
    selector: 'app-setup-dentists',
    imports: [
        MatCard,
        MatCardHeader,
        MatCardTitle,
        MatCardSubtitle,
        MatCardContent,
        GenericDataTableComponent,
        MatButton
    ],
    templateUrl: './setup-dentists.html',
    styleUrl: './setup-dentists.scss',
    standalone: true
})
export class SetupDentists {
    private readonly router = inject(Router);
    private readonly destroyRef = inject(DestroyRef);
    private readonly dentistService = inject(DentistService);
    private readonly dentistClinicService = inject(DentistClinicService);

    dentists = signal<DentistWithLookups[]|null>(null);
    dentist_clinics = signal<DentistClinicWithNames[]>([]);
    dentistsAllInfo = signal<DentistWithLookupsAndClinicInfo[]>([]);
    readonly columns: TableColumn<DentistWithLookupsAndClinicInfo>[] = [
        { key: 'id', label: 'ID' },
        { key: 'last_name', label: 'Last Name' },
        { key: 'given_name', label: 'Given Name' },
        { key: 'middle_name', label: 'Middle Name' },
        { key: 'dentist_contract_name', label: 'Dentist Contract' },
        { key: 'prc_no', label: 'PRC License' },
        { key: 'prc_expiry_date', label: 'PRC Expiry Date' },
        { key: 'dentist_status_name', label: 'Status'},
        { key: 'dentist_history_name', label: 'History'},
    ];
    constructor(){}

    ngOnInit(){
        forkJoin({
            dentists: this.dentistService.getAllDentists(),
            dentist_clinics: this.dentistClinicService.getAllDentistClinics(),
        })
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: ({ dentists, dentist_clinics }) => {
                    // set signals
                    this.dentists.set(dentists);
                    this.dentist_clinics.set(dentist_clinics);

                    console.log('In ngOnInit(), dentists:', dentists);
                    console.log('In ngOnInit(), dentist_clinics:', dentist_clinics);

                    // now both are available
                    const dentistAllInfo = this.buildDentistWithLookupAndClinicInfo(dentists, dentist_clinics);
                    this.dentistsAllInfo.set(dentistAllInfo);
                    console.log('In ngOnInit(), dentistAllInfo:', dentistAllInfo);
                },
                error: (err) => {
                    console.log('In load(), failed to load data', err);
                },
            });
    }


    onNewDentist(){
        this.router.navigate(['/main/setup/dentists/new']).then();
    }

    onRowClicked(row: DentistWithLookups) {
        console.log("In onRowClicked(), row:", row);
        // Route example: /setup/dental-clinics/:id
        this.router.navigate(['/main/setup/dentists/', row.id]).then();
    }

    buildDentistWithLookupAndClinicInfo(
        dentists: readonly DentistWithLookups[],
        dentistClinics: readonly DentistClinicWithNames[])
    : DentistWithLookupsAndClinicInfo[]{
        // Pick ONE clinic row per dentist_id (first one wins).
        const byDentistId = new Map<number, DentistClinicWithNames>();
        for (const dc of dentistClinics) {
            if (!byDentistId.has(dc.dentist_id)) byDentistId.set(dc.dentist_id, dc);
        }

        // Copy dentist rows + attach clinic fields (or nulls if none).
        return dentists.map((d) => {
            const dc = byDentistId.get(d.id);

            return {
                ...d,
                clinic_name: dc?.clinic_name ?? null,
                position: dc?.position ?? null,
                schedule: dc?.schedule ?? null,
            };
        });



    }
}
