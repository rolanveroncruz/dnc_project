import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {GenericDataTableComponent} from "../../../components/generic-data-table-component/generic-data-table-component";
import {MatButton} from "@angular/material/button";
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from "@angular/material/card";
import {Router} from '@angular/router';
import {EndorsementService, EndorsementListRow} from '../../../api_services/endorsement-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {DentistWithLookupsAndClinicInfo} from '../setup-dentists/setup-dentists';

@Component({
  selector: 'app-setup-endorsements',
    imports: [
        GenericDataTableComponent,
        MatButton,
        MatCard,
        MatCardContent,
        MatCardHeader,
        MatCardSubtitle,
        MatCardTitle
    ],
  templateUrl: './setup-endorsements.html',
  styleUrl: './setup-endorsements.scss',
})
export class SetupEndorsements implements OnInit{
    private readonly router = inject(Router);
    private readonly destroyRef = inject(DestroyRef);
    private endorsementsService = inject(EndorsementService);

    endorsements = signal<EndorsementListRow[] | null>(null);


    constructor(){

    }

    readonly columns: TableColumn<EndorsementListRow>[] = [
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
    ngOnInit(): void {
        this.endorsementsService.getAll()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (res)=>{
                    this.endorsements.set(res.items);
                },
                error: (err)=>{
                    console.log("In load(), failed to load users", err);
                }
            })
    }

    onNewEndorsement(){
        this.router.navigate(['/main/setup/endorsements/new']).then();
    }
    onRowClicked(row: EndorsementListRow){
        this.router.navigate(['/main/setup/endorsements/', row.id]).then();
    }
}
