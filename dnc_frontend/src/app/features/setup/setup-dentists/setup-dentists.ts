import {Component, DestroyRef, inject, signal} from '@angular/core';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {Router} from '@angular/router';
import {DentistService, DentistWithLookups} from '../../../api_services/dentist-service';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {DentalClinicRow} from '../setup-dental-clinics-component/setup-dental-clinics-component';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {MatButton} from '@angular/material/button';

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

    dentists = signal<DentistWithLookups[]|null>(null);
    readonly columns: TableColumn<DentalClinicRow>[] = [
        { key: 'id', label: 'ID' },
        { key: 'last_name', label: 'Last Name' },
        { key: 'given_name', label: 'Given Name' },
        { key: 'middle_name', label: 'Middle Name' },
        { key: 'dentist_contract_name', label: 'Dentist Contract' },
        { key: 'region_name', label: 'Region' },
        { key: 'contact_numbers', label: 'Contact' },
        { key: 'hasPanoramic', label: 'Panoramic Radio', cellTemplateKey: 'check'},
        { key: 'hasPeriapical', label: 'Periapical Radio', cellTemplateKey: 'check'},
    ];
    constructor(){}

    ngOnInit(){
        this.dentistService.getAllDentists()
        .pipe(takeUntilDestroyed(this.destroyRef))
        .subscribe({
            next: (dentists) => {
                this.dentists.set(dentists);
            },
            error: (err) => {
                console.log("In load(), failed to load users", err);
            }
        })
    }


    onNewDentist(){
        this.router.navigate(['/main/setup/dentists/new']).then();
    }

    onRowClicked(row: DentistWithLookups) {
        console.log("In onRowClicked(), row:", row);
        // Route example: /setup/dental-clinics/:id
        this.router.navigate(['/main/setup/dentists/', row.id]).then();
    }

}
