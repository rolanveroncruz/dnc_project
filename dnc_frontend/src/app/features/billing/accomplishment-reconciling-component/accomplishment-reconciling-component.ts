import {Component, inject, OnInit, signal} from '@angular/core';
import {CreateAccReconciliationRequest, AccomplishmentReconciliationService, DoneVerificationResponse} from '../../../api_services/accomplishment-reconciliation-service';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {MatButton} from '@angular/material/button';
import {MatDialog} from '@angular/material/dialog';
import {AddAccReconciliationDialog, AddAccReconciliationDialogData,  IdLabelOption} from './add-acc-reconciliation-dialog/add-acc-reconciliation-dialog';
import {DentistService} from '../../../api_services/dentist-service';
import {DentalServicesService} from '../../../api_services/dental-services-service';
import {VerificationService} from '../../../api_services/verification-service';
import {EndorsementCompanyOptions, EndorsementService} from '../../../api_services/endorsement-service';
import {finalize} from 'rxjs/operators';

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
export class AccomplishmentReconcilingComponent implements OnInit {
    readonly accomplishment_reconciliation_service = inject(AccomplishmentReconciliationService);
    dialog = inject(MatDialog);
    readonly done_verifications = signal<DoneVerificationResponse[]>([]);
    readonly accomplishment_reconciliations = signal<DoneVerificationResponse[]>([]);
    readonly updatingVerificationIds = signal<ReadonlySet<number>>(new Set<number>());

    // region: Dialog Data Services
    endorsementService = inject(EndorsementService);
    dentistService = inject(DentistService);
    dentalServicesService = inject(DentalServicesService);
    verificationService = inject(VerificationService);


    // region: Dialog Data
    dentists = signal<IdLabelOption[]>([]);
    companies = signal<IdLabelOption[]>([]);
    dentalServices = signal<IdLabelOption[]>([]);
    toothServiceTypes = signal<IdLabelOption[]>([]);
    toothSurfaces = signal<IdLabelOption[]>([]);
    // endregion: Dialog Data

    DoneVerificationsColumns: TableColumn<DoneVerificationResponse>[] = [
        {key: 'id', label: 'ID'},
        {key: 'agreement_corp_number', label: 'Agmt/Corp Number'},
        {key: 'company_name', label: 'Company'},
        {key: 'dentist_name', label: 'Dentist'},
        {key: 'member_name', label: 'Member'},
        {key: 'dental_service_name', label: 'Procedure',},
        {key: 'tooth_id', label: 'Tooth'},
        {key: 'date_service_performed', label: 'Service Date'},
        {key: 'approval_code', label: 'Approval Code'},
        {
            // ✅ CHANGED: The last column now displays a reconciliation checkbox
            key: 'is_reconciled',
            label: 'Reconciled',
            sortable: true,
            widthPx: 120,
            cellTemplateKey: 'checkbox',

            // ✅ NEW: Interactive checkbox configuration
            checkbox: {
                checked: (row: DoneVerificationResponse): boolean =>
                    row.is_reconciled,

                disabled: (row: DoneVerificationResponse): boolean =>
                    this.isVerificationUpdating(row),

                onChange: (
                    row: DoneVerificationResponse,
                    checked: boolean
                ): void => {
                    this.onReconciledCheckboxChange(row, checked);
                },
            },
        }
    ];
    AddlAccomplishmentReportsCols: TableColumn[] = [
        {key: 'id', label: 'ID'},
        {key: 'agreement_corp_number', label: 'Agmt/Corp Number'},
        {key: 'company_name', label: 'Company'},
        {key: 'dentist_name', label: 'Dentist'},
        {key: 'member_name', label: 'Member'},
        {key: 'dental_service_name', label: 'Procedure',},
        {key: 'date_service_performed', label: 'Service Date'},
        {key: 'approval_code', label: 'Approval Code'},
    ];

    ngOnInit(): void {
        this.loadDoneVerifications();
        this.loadAccReconciliations();
        this.loadDentists();
        this.loadCompanies();
        this.loadDentalServices();
        this.loadToothServiceTypes();
        this.loadToothSurfaces();


    }

    loadDoneVerifications() {
        this.accomplishment_reconciliation_service.getDoneVerifications().subscribe({
            next: (res) => {
                this.done_verifications.set(res);
            },
            error: (err) => {
                console.log("In load(), failed to load done verifications", err);
            }
        })
    }

    loadAccReconciliations(){
        this.accomplishment_reconciliation_service.getAccReconciliation()
            .subscribe({
                next: (res)=>{
                    this.accomplishment_reconciliations.set(res);
                },
                error: (err)=>{
                    console.log("In loadAccReconciliations(), failed to load accomplishment reconciliations", err);
                }
            })
    }

    // region: Load Dialog Data
    loadDentists(){
        this.dentistService.getAllDentistsNamesOnly().subscribe({
            next: (res) => {
                const dentistOptions: IdLabelOption[] = res.map((dentist: any) => ({
                    id: dentist.id,
                    label: dentist.full_name,
                }));
                this.dentists.set(dentistOptions);
            },
            error: (err) => {
                console.log("In loadDentists(), failed to load dentists", err);
            },
        });
    }

    loadCompanies(){
        this.endorsementService.getEndorsementCompanies().subscribe({
                next: (res) => {

                    const companies: IdLabelOption[] = res.map((company: EndorsementCompanyOptions)=>({
                        id: company.id,
                        label: company.name,
                    }));
                    this.companies.set(companies);
                },
                error: (err) => {
                    console.log("In loadCompanies(), failed to load companies", err);
                }
            }
        )

    }

    loadDentalServices(){
         this.dentalServicesService.getDentalServices().subscribe({
             next: (res)=>{
                 const dentalServices = res.items.map((service: any)=>({
                     id: service.id,
                     label: service.name,
                 }))
                 this.dentalServices.set(dentalServices);
             } ,
             error: (err)=>{
                 console.log("In loadDentalServices(), failed to load dental services", err);
             }
         })
    }

    loadToothServiceTypes(){
        this.verificationService.getToothServiceType().subscribe({
            next: (res)=> {
                const toothServiceTypes = res.map((type: any)=>({
                    id: type.id,
                    label: type.name,
                }))
                this.toothServiceTypes.set(toothServiceTypes);
            },
            error: (err)=> {
                console.log("In loadToothServiceTypes(), failed to load tooth service types", err);
            }
        });

    }

    loadToothSurfaces(){
        this.verificationService.getToothSurfaces().subscribe({
            next: (res)=> {
                const toothSurfaces = res.map((surface: any)=>({
                    id: surface.id,
                    label: surface.name,
                }))
                this.toothSurfaces.set(toothSurfaces);
            },
            error: (err)=> {
                console.log("In loadToothSurfaces(), failed to load tooth surfaces", err);
            }
        });

    }

    // endregion: Load Dialog Data

    AlwaysHide = (_row: any): boolean => true;

    // ✅ NEW: Used by the checkbox column to disable a row while saving
    isVerificationUpdating = (
        row: DoneVerificationResponse
    ): boolean => {
        return this.updatingVerificationIds().has(row.id);
    };

    // ✅ NEW: Adds or removes a verification ID from the updating set
    private setVerificationUpdating(
        verificationId: number,
        updating: boolean
    ): void {
        this.updatingVerificationIds.update(currentIds => {
            const nextIds = new Set(currentIds);

            if (updating) {
                nextIds.add(verificationId);
            } else {
                nextIds.delete(verificationId);
            }

            return nextIds;
        });
    }

// ✅ NEW: Replaces one verification row without reloading the whole table
    private updateDoneVerificationRow(
        updatedRow: DoneVerificationResponse
    ): void {
        this.done_verifications.update(rows =>
            rows.map(row =>
                row.id === updatedRow.id
                    ? updatedRow
                    : row
            )
        );
    }

// ✅ CHANGED: Reconciles or unreconciles based on the checkbox state
    onReconciledCheckboxChange(
        row: DoneVerificationResponse,
        checked: boolean
    ): void {
        // ✅ Save the original value so it can be restored on failure
        const originalReconciledState = row.is_reconciled;

        // ✅ Prevent duplicate requests for the same verification
        if (this.isVerificationUpdating(row)) {
            return;
        }

        // ✅ Disable the checkbox while the request is running
        this.setVerificationUpdating(row.id, true);

        // ✅ Optimistically update the checkbox display
        this.done_verifications.update(rows =>
            rows.map(currentRow =>
                currentRow.id === row.id
                    ? {
                        ...currentRow,
                        is_reconciled: checked,
                    }
                    : currentRow
            )
        );

        // ✅ Select the correct backend request
        const request$ = checked
            ? this.accomplishment_reconciliation_service
                .reconcileVerification(row.id)
            : this.accomplishment_reconciliation_service
                .unreconcileVerification(row.id);

        request$
            .pipe(
                // ✅ Always re-enable the checkbox afterward
                finalize(() => {
                    this.setVerificationUpdating(row.id, false);
                })
            )
            .subscribe({
                next: (updatedVerification) => {
                    // ✅ Replace the row with the authoritative backend response
                    this.updateDoneVerificationRow(updatedVerification);
                },
                error: (err) => {
                    console.error(
                        `Failed to ${
                            checked ? 'reconcile' : 'unreconcile'
                        } verification ${row.id}`,
                        err
                    );

                    // ✅ Restore the original checkbox state
                    this.done_verifications.update(rows =>
                        rows.map(currentRow =>
                            currentRow.id === row.id
                                ? {
                                    ...currentRow,
                                    is_reconciled: originalReconciledState,
                                }
                                : currentRow
                        )
                    );
                },
            });
    }


    addAccomplishment() {
        console.log("In addAccomplishment(). Opening dialog...");
        const ref = this.dialog.open<
            AddAccReconciliationDialog,
            AddAccReconciliationDialogData,
            CreateAccReconciliationRequest|null>(
                AddAccReconciliationDialog, {
                    width: '720px',
                maxWidth: '95vw',
                disableClose: true,
                data:{
                    companies: this.companies(),
                    dentists: this.dentists(),
                    dental_services: this.dentalServices(),
                    tooth_service_types: this.toothServiceTypes(),
                    tooth_surfaces: this.toothSurfaces(),
                },
            });
        ref.afterClosed().subscribe(result=>{
            if (!result) return;
            console.log("Result from dialog:", result);
            this.accomplishment_reconciliation_service.postAccReconciliation(result).subscribe({
                next: (res)=>{
                    console.log("In postAccReconciliation(), res:", res);
                    this.loadDoneVerifications();
                },
                error: (err)=>{
                    console.log("In postAccReconciliation(), failed to post accomplishment", err);
                }
            })

        })


    }
}
