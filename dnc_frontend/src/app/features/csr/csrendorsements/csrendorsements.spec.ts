import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CSREndorsements } from './csrendorsements';

describe('CSREndorsements', () => {
  let component: CSREndorsements;
  let fixture: ComponentFixture<CSREndorsements>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CSREndorsements]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CSREndorsements);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
