import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {VerificationService, VerificationLookupResponse} from '../../../api_services/verification-service';
import {Router} from '@angular/router';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {MatButton} from '@angular/material/button';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {MatDialog} from '@angular/material/dialog';
import {SimpleConfirmDialogComponent} from '../../../components/simple-confirm-dialog-component/simple-confirm-dialog-component';

@Component({
  selector: 'app-verifications-component',
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
  templateUrl: './verifications-component.html',
  styleUrl: './verifications-component.scss',
})
export class VerificationsComponent implements OnInit {
    private readonly router = inject(Router);
    private readonly dialog = inject(MatDialog);
    private readonly verificationService = inject(VerificationService);
    private readonly destroyRef = inject(DestroyRef);
    verifications = signal<VerificationLookupResponse[]>([]);

    getLabelFromRow(row: VerificationLookupResponse): string {
        return row.status_name;
    }
    readonly columns: TableColumn<VerificationLookupResponse>[] = [
        { key: 'verification_id', label: 'ID' },
        { key: 'date_created', label: 'Date', cellTemplateKey: 'date' },
        { key: 'dentist_name', label: 'Dentist'},
        { key: 'master_list_member_name', label: 'Member'},
        {key: 'dental_service_name', label: 'Service'},
        {key: 'status_name', label: 'Status'},
        { key: 'actions', label:'Actions', sortable: false, cellTemplateKey: 'actions',
            actionButton: {
                label: this.getRowLabel,
                icon: this.getRowIcon,
                color: 'primary',
                hidden: (row: VerificationLookupResponse) => row.status_id==0,
                onClick: function (row: VerificationLookupResponse): void {
                    console.log("In onActionButtonClicked(), row:", row);
                }
            },
        }
    ]
    getRowLabel(row: VerificationLookupResponse): string {
        return row.status_id==1 ? 'Approval Code' : 'Upload Files';
    }

    getRowIcon(row: VerificationLookupResponse): string {
        return row.status_id==1 ? 'check' : 'upload';
    }

    onActionButtonClicked(row:VerificationLookupResponse): void {
        console.log("In onActionButtonClicked(), row:", row);
    }

    isSecondaryActionHidden(row: VerificationLookupResponse): boolean {
        return row.status_id ===0;
    }

    ngOnInit(): void {
        this.loadVerifications();

    }

    loadVerifications(){
        this.verificationService.getVerifications()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next:(res)=> this.verifications.set(res),
                error:(err)=> console.log("In load(), failed to load verifications",err )
            })

    }

    onNewVerification(){
        const url = this.router.serializeUrl(this.router.createUrlTree(['main/csr/verifications/new']));
        window.open(url, '_blank');
    }

    onRowClicked(row: VerificationLookupResponse) {
        console.log("In onRowClicked(), row:", row);
        // Route example: /setup/dental-clinics/:id
        this.router.navigate(['main/csr/verifications/', row.verification_id]).then();
    }

    onCancelVerification(row: VerificationLookupResponse){
        const memberName = row.master_list_member_name || `Member #${row.master_list_member_id}`;
        const serviceName = row.dental_service_name || `Service #${row.dental_service_id}`;

        this.dialog.open(SimpleConfirmDialogComponent, {
            width: '400px',
            data: {
                title: 'Cancel Verification',
                message: `Are you sure you want to cancel verification for ${memberName} for ${serviceName}?`,
                confirmButtonText: 'Cancel Verification',
                cancelButtonText: 'Keep Verification',
            }
        }).afterClosed().subscribe((confirmed:boolean) => {
            if (!confirmed){
                return;
            }
            this.verificationService.cancelVerification(row.verification_id).subscribe({
                next: (res) => {
                    console.log("In onCancelVerification(), cancelled verification for ", memberName, " for ", serviceName);
                    this.loadVerifications();
                },
                error: (err) => {
                    console.log("In onCancelVerification(), failed to cancel verification for ", memberName, " for ", serviceName, err);
                }
            });

        });


    }

}
