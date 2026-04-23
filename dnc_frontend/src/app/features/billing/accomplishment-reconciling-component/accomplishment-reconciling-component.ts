import {Component, inject, OnInit, signal} from '@angular/core';
import {AccomplishmentReconciliationService, DoneVerificationResponse} from '../../../api_services/accomplishment-reconciliation-service';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {MatButton} from '@angular/material/button';
import {MatDialog} from '@angular/material/dialog';
import {AddAccReconciliationDialog, AddAccReconciliationDialogData, CreateAccReconciliationRequest, IdLabelOption} from './add-acc-reconciliation-dialog/add-acc-reconciliation-dialog';
import {DentistService} from '../../../api_services/dentist-service';
import {DentalServicesService} from '../../../api_services/dental-services-service';
import {VerificationService} from '../../../api_services/verification-service';
import {EndorsementCompanyOptions, EndorsementService} from '../../../api_services/endorsement-service';

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

    DoneVerificationsColumns: TableColumn[] = [
        {key: 'id', label: 'ID'},
        {key: 'agreement_corp_number', label: 'Agmt/Corp Number'},
        {key: 'company_name', label: 'Company'},
        {key: 'dentist_name', label: 'Dentist'},
        {key: 'member_name', label: 'Member'},
        {key: 'dental_service_name', label: 'Procedure',},
        {key: 'date_service_performed', label: 'Service Date'},
        {key: 'approval_code', label: 'Approval Code'},
        {
            key: 'actions', label: 'Actions', cellTemplateKey: 'actions',
            actionButton: {
                label: "Confirm",
                icon: 'check',
                color: 'primary',
                onClick: (row: any) => console.log("In onConfirmClick() onClick, row:", row),
                hidden: (row: any) => row.is_reconciled,
            }
        }
    ];

    ngOnInit(): void {
        this.loadDoneVerifications();
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


    onConfirmClick(row: any) {
        console.log("OnConfirmClick(), row id:", row.id);

        this.accomplishment_reconciliation_service.reconcileVerification(row.id)
            .subscribe({
                next: (res) => {
                    console.log("In reconcileVerification(), res:", res);
                    this.loadDoneVerifications();
                },
                error: (err) => {
                    console.log("In reconcileVerification(), failed to reconcile verification", err);
                }
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
