import {Component, inject, OnInit, signal} from '@angular/core';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {MatButton} from '@angular/material/button';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {HighEndVerificationsService, HighEndVerificationResponse, HighEndFileResponse} from '../../../api_services/high-end-verifications-service';
import {MatDialog} from '@angular/material/dialog';
import {
    DentistHighEndApprovalDialogComponent
} from './dentist-high-end-approval-dialog/dentist-high-end-approval-dialog';

@Component({
  selector: 'app-high-end-verification',
    imports: [
        GenericDataTableComponent,
        MatButton,
        MatCard,
        MatCardContent,
        MatCardHeader,
        MatCardSubtitle,
        MatCardTitle
    ],
  templateUrl: './high-end-verification.html',
  styleUrl: './high-end-verification.scss',
})
export class HighEndVerification implements OnInit {
    private readonly highEndVerificationsService = inject(HighEndVerificationsService);
    private readonly dialog = inject(MatDialog);


    high_end_verifications = signal<HighEndVerificationResponse[]>([]);

    readonly columns: TableColumn<HighEndVerificationResponse>[] = [
        {key: 'verification_id', label: 'ID'},
        {key: 'date_created', label: 'Date', cellTemplateKey: 'date'},
        {key: 'hmo_name', label: 'HMO'},
        {key: 'dentist_name', label: 'Dentist'},
        {key: 'member_name', label: 'Member'},
        {key: 'dental_service_name', label: 'Service'},
        {key: 'status_name', label: 'Status'},
    ]

    constructor() {
    }

    ngOnInit(): void {
        this.highEndVerificationsService
            .getAllHighEndVerifications()
            .subscribe({
                next: (res) => {
                    this.high_end_verifications.set(res);
                },
                error: (err) => {
                    console.log("In load(), failed to load high end verifications", err);
                }
            })

    }

    hideSecondary(){
        return true;
    }

    onRowClicked(res: HighEndVerificationResponse) {
        console.log("In onRowClicked(), res:", res);
        this.dialog.open(DentistHighEndApprovalDialogComponent,{
            data: res,
            width: '720px',
            maxWidth: '95vw',
        })
    }
}


