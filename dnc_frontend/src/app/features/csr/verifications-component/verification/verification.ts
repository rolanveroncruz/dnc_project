import {Component, computed, inject, OnDestroy, OnInit, signal} from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { DentistService, DentistNames } from '../../../../api_services/dentist-service';
import { MatInput } from '@angular/material/input';
import {MatFormField, MatLabel} from '@angular/material/form-field';
import {FormControl, ReactiveFormsModule} from '@angular/forms';
import {MatAutocomplete, MatAutocompleteSelectedEvent, MatAutocompleteTrigger} from '@angular/material/autocomplete';
import {MatOptionModule} from '@angular/material/core';
import {MasterListMemberComponent} from './master-list-member-component/master-list-member-component';
import {MemberServicesCountsService, MemberServicesCountsSummary} from '../../../../api_services/member-services-counts-service';
import {ServicesComponent} from './services-component/services-component';
import {MatButtonModule} from '@angular/material/button';
import {forkJoin} from 'rxjs';
import {CreateVerificationResponse, VerificationService} from '../../../../api_services/verification-service';
import {DatePipe} from '@angular/common';

@Component({
    selector: 'app-verification',
    standalone: true,
    imports: [
        MatFormField,
        MatLabel,
        MatAutocomplete,
        ReactiveFormsModule,
        MatOptionModule,
        MatAutocompleteTrigger,
        MatInput,
        MasterListMemberComponent,
        ServicesComponent,
        MatButtonModule,
        DatePipe,
    ],
    templateUrl: './verification.html',
    styleUrl: './verification.scss',
})
export class Verification implements OnInit,OnDestroy {
    private readonly route = inject(ActivatedRoute);
    readonly verificationService = inject(VerificationService);

    // region: Current date/time and saving
    readonly currentDateTime = signal(new Date());
    readonly saving = signal(false);
    readonly saveError = signal<string | null>(null);
    readonly saveSuccessMessage = signal<string |null>(null);
    private clockIntervalId: ReturnType<typeof setInterval>|null = null;
    // endregion: Current date/time and saving


// region: Services and Service Counts,
    /******************
     * Services and Service Counts
     ****************/

    private readonly memberServicesCountsService = inject(MemberServicesCountsService);
    readonly memberServicesCountsSummary = signal<MemberServicesCountsSummary[]>([]);
    readonly selectedDentalServiceIds = signal<number[]>([]);
    readonly loadingMemberServices = signal(false);

    onCheckedServiceIdsChange(ids: number[]): void {
        this.selectedDentalServiceIds.set(ids);
        this.saveError.set(null);
        this.saveSuccessMessage.set(null);
    }

    private loadMemberServicesCounts(memberId: number): void {
        console.log("In loadMemberServicesCounts(), memberId:", memberId);
        this.loadingMemberServices.set(true);

        this.memberServicesCountsService.getMemberServicesCountsSummary(memberId).subscribe({
            next: (res) => {
                this.memberServicesCountsSummary.set(res);
                this.loadingMemberServices.set(false);
            },
            error: (err) => {
                console.error('Failed to load member service counts summary', err);
                this.memberServicesCountsSummary.set([]);
                this.loadingMemberServices.set(false);
            }
        });
    }
    // endregion: Services and Service Counts,

// region: Route path ID, and signals for if new or edit mode
    /******************
     * Route path ID, and signals for if new or edit mode
     ****************/
    readonly routePath = signal<string | undefined>(undefined);
    readonly routeIdParam = signal<string | null>(null);
    readonly isCreateMode = computed(() => this.routePath() === 'verifications/new');
    readonly verificationId = computed<number | null>(() => {
        if (this.routePath() !== 'verifications/:id') {
            return null;
        }

        const raw = this.routeIdParam();
        if (!raw) {
            return null;
        }

        const id = Number(raw);
        return Number.isInteger(id) && id > 0 ? id : null;
    });
    readonly isEditMode = computed(() => this.routePath() === 'verifications/:id' && this.verificationId() !== null);
    readonly isInvalidMode = computed(() => {
        if (this.isCreateMode()) {
            return false;
        }

        if (this.routePath() === 'verifications/:id') {
            return this.verificationId() === null;
        }

        return true;
    });
// endregion: Route path ID, and signals for if new or edit mode



// region: For the dentist autocomplete control
    /***************
    * For dentist
     ***************/
    // These are to pull the dentists' data
    readonly dentistService = inject(DentistService);
    readonly dentistNames = signal<DentistNames[] >([]);
    // just an indicator
    readonly loadingDentists = signal<boolean>(false);
    // dentistSearch is the text box value. in ngOnInit(), we subscribe to the valueChanges and set the dentistSearchText signal
    // to the value of dentistSearch's value.
    // filteredDentists is the array then computed based on the dentistSearchText signal.
    readonly dentistSearch = new FormControl<string|number>('', { nonNullable: true });
    readonly selectedDentistId = signal<number | null>(null);
    readonly dentistSearchText = signal('');
    readonly filteredDentists = computed(() => {

        const search = this.dentistSearchText();

        if (!search) {
            return this.dentistNames();
        }

        return this.dentistNames().filter(dentist =>
            dentist.full_name.toLowerCase().includes(search)
        );
    });

    onDentistSelected(event: MatAutocompleteSelectedEvent): void {
        const selectedId = event.option.value as number;
        this.selectedDentistId.set(selectedId);

        // reset the selectedMasterListMemberId
        this.selectedMasterListMemberId.set(null);
        this.memberServicesCountsSummary.set([]);
        this.selectedDentalServiceIds.set([]);
        this.saveError.set(null);
        this.saveSuccessMessage.set(null);

        const selectedDentist = this.dentistNames().find(d => d.id === selectedId);
        if (selectedDentist) {
            this.dentistSearch.setValue(selectedDentist.full_name, { emitEvent: false });
        }
    }

    // endregion: For the dentist autocomplete control

// region for MasterListMember

    readonly selectedMasterListMemberId = signal<number | null>(null);

    onMasterListMemberResolved(memberId: number|null):void{
        console.log("In onMasterListMemberResolved(), memberId:", memberId);
        this.selectedMasterListMemberId.set(memberId);
        this.saveError.set(null);
        this.saveSuccessMessage.set(null);

        //clear previous service selection first
        this.memberServicesCountsSummary.set([]);
        this.selectedDentalServiceIds.set([]);
        if(memberId === null){
            return;
        }
        this.loadMemberServicesCounts(memberId);
    }

    // endregion for MasterListMember

    // region: constructor(), ngOnInit, ngOnDestroy

    constructor() {
        this.route.url.subscribe(() => {
            this.routePath.set(this.route.routeConfig?.path);
            this.routeIdParam.set(this.route.snapshot.paramMap.get('id'));
        });
    }

    ngOnInit(): void {
        this.loadDentists();

        this.clockIntervalId = setInterval(() => {
            this.currentDateTime.set(new Date());
        }, 1000);

        this.dentistSearch.valueChanges.subscribe(value => {
            // value can be string while typing, or number after autocomplete selection
            const searchText = typeof value ==='string'? value.trim().toLowerCase(): '';
            this.dentistSearchText.set(searchText);

            if (typeof value === 'string') {
                this.saveError.set(null);
                this.saveSuccessMessage.set(null);

                this.selectedDentistId.set(null);
                this.selectedMasterListMemberId.set(null);
                this.memberServicesCountsSummary.set([]);
                this.selectedDentalServiceIds.set([]);
            }
        });

    }

    ngOnDestroy(): void {
        if (this.clockIntervalId) {
            clearInterval(this.clockIntervalId);
        }
    }


    //endregion: constructor(), ngOnInit, ngOnDestroy
    private loadDentists():void{
        this.loadingDentists.set(true);

        this.dentistService.getAllDentistsNamesOnly().subscribe({
            next: (res)=>{
                this.dentistNames.set(res);
                this.loadingDentists.set(false);
            },
            error: (err)=>{
                console.log("In loadDentists(), failed to load dentists", err);
                this.loadingDentists.set(false);
            }
        })
    };

    // region: Saving
    readonly canSave = computed(() =>
        this.isCreateMode() &&
        this.selectedDentistId() !== null &&
        this.selectedMasterListMemberId() !== null &&
        this.selectedDentalServiceIds().length > 0 &&
        !this.saving()
    );

    saveVerifications(): void {
        const dentistId = this.selectedDentistId();
        const memberId = this.selectedMasterListMemberId();
        const serviceIds = this.selectedDentalServiceIds();

        if (dentistId === null || memberId === null || serviceIds.length === 0) {
            this.saveError.set('Please select a dentist, a member, and at least one service.');
            this.saveSuccessMessage.set(null);
            return;
        }

        this.saving.set(true);
        this.saveError.set(null);
        this.saveSuccessMessage.set(null);

        const requests = serviceIds.map(dental_service_id =>
            this.verificationService.createVerification({
                    dentist_id: dentistId,
                    member_id: memberId,
                    dental_service_id: dental_service_id,
                }
            ));

        forkJoin(requests).subscribe({
            next: (responses: CreateVerificationResponse[]) => {
                this.saving.set(false);
                this.saveSuccessMessage.set(
                    `${responses.length} verification(s) created successfully.`
                );
            },
            error: (err) => {
                console.error('Failed to create verifications', err);
                this.saving.set(false);
                this.saveError.set('Failed to save verification(s).');
            }
        });
    }
    // endregion: Saving
}
