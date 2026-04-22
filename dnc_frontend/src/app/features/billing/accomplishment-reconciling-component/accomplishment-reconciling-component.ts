import {Component, inject, OnInit, signal} from '@angular/core';
import {AccomplishmentReconciliationService, DoneVerificationResponse} from '../../../api_services/accomplishment-reconciliation-service';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {MatButton} from '@angular/material/button';

@Component({
  selector: 'app-accomplishment-reconciling-component',
    imports: [
        GenericDataTableComponent,
        MatCard,
        MatCardContent,
        MatCardHeader,
        MatCardSubtitle,
        MatCardTitle,
        MatButton
    ],
  templateUrl: './accomplishment-reconciling-component.html',
  styleUrl: './accomplishment-reconciling-component.scss',
})
export class AccomplishmentReconcilingComponent implements OnInit{
    readonly accomplishment_reconciliation_service = inject(AccomplishmentReconciliationService);

    readonly done_verifications = signal<DoneVerificationResponse[]>([]);

    DoneVerificationsColumns: TableColumn[] = [
        {key: 'id', label: 'ID'},
        {key: 'agreement_corp_number', label: 'Agmt/Corp Number'},
        {key: 'company_name', label: 'Company'},
        {key: 'dentist_name', label: 'Dentist'},
        {key: 'member_name', label: 'Member'},
        {key: 'dental_service_name', label: 'Procedure',},
        {key: 'date_service_performed', label: 'Service Date'},
        {key: 'approval_code', label: 'Approval Code'},
        {key: 'actions',label: 'Actions', cellTemplateKey: 'actions',
            actionButton: {
                label: "Confirm",
                icon: 'check',
                color: 'primary',
                onClick: (row:any)=> console.log("In onConfirmClick() onClick, row:", row),
                hidden: (row:any)=> row.is_reconciled,
            }
        }
    ];

    ngOnInit(): void {
        this.loadDoneVerifications();
    }
    loadDoneVerifications(){
        this.accomplishment_reconciliation_service.getDoneVerifications().subscribe({
            next: (res)=>{
                this.done_verifications.set(res);
            },
            error: (err)=>{
                console.log("In load(), failed to load done verifications", err);
            }
        })
    }
    AlwaysHide = (_row: any):boolean => true;


    addAccomplishment(){
        console.log("In addAccomplishment()");

    }
    onConfirmClick( row:any){
        console.log("OnConfirmClick(), row id:", row.id);

        this.accomplishment_reconciliation_service.reconcileVerification(row.id)
            .subscribe({
                next: (res)=>{
                    console.log("In reconcileVerification(), res:", res);
                    this.loadDoneVerifications();
                },
                error: (err)=>{
                    console.log("In reconcileVerification(), failed to reconcile verification", err);
                }
            });
    }
}
