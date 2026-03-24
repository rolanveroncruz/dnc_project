import { Component, computed, inject,OnInit, signal } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { MasterListMemberService } from '../../../../api_services/master-list-members-service';
import { DentistService, DentistNames } from '../../../../api_services/dentist-service';
import { MatInput } from '@angular/material/input';
import {MatFormField, MatLabel} from '@angular/material/form-field';
import {FormControl, ReactiveFormsModule} from '@angular/forms';
import {MatAutocomplete, MatAutocompleteSelectedEvent, MatAutocompleteTrigger} from '@angular/material/autocomplete';
import {MatOptionModule} from '@angular/material/core';

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
    ],
    templateUrl: './verification.html',
    styleUrl: './verification.scss',
})
export class Verification implements OnInit {
    private readonly route = inject(ActivatedRoute);
    readonly masterListMemberService = inject(MasterListMemberService);


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
    // filteredDentists is then computed based on the dentistSearchText signal.
    readonly dentistSearch = new FormControl<string>('', { nonNullable: true });
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

    constructor() {
        this.route.url.subscribe(() => {
            this.routePath.set(this.route.routeConfig?.path);
            this.routeIdParam.set(this.route.snapshot.paramMap.get('id'));
        });
    }

    onDentistSelected(event: MatAutocompleteSelectedEvent): void {
        const selectedId = event.option.value as number;
        this.selectedDentistId.set(selectedId);

        const selectedDentist = this.dentistNames().find(d => d.id === selectedId);
        if (selectedDentist) {
            this.dentistSearch.setValue(selectedDentist.full_name, { emitEvent: false });
        }
    }
    ngOnInit(): void {
        this.loadDentists();
        this.dentistSearch.valueChanges.subscribe(value => {
            this.dentistSearchText.set((value ?? '').trim().toLowerCase());
        });

    }

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
}
