import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {GenericDataTableComponent} from "../../../components/generic-data-table-component/generic-data-table-component";
import {MatButton} from "@angular/material/button";
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from "@angular/material/card";
import {Router} from '@angular/router';
import {EndorsementService, EndorsementListRow} from '../../../api_services/endorsement-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';

@Component({
  selector: 'app-setup-endorsements',
    standalone: true,
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
        { key: 'hmo_name', label: 'HMO' },
        { key: 'company_name', label: 'Company' },
        { key: 'date_start', label: 'Date Start', cellTemplateKey: 'date' },
        { key: 'date_end', label: 'Date End', cellTemplateKey: 'date' },
        { key: 'type_name', label: 'Type' },
        { key: 'billing_period_type_name', label: 'Billing Period' },
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
